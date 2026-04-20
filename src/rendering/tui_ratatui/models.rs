use std::io::Stdout;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    common::{
        constants::{LIBRARY_LIST_SECTION_NAME, LIBRARY_METADATA_SECTION_NAME},
        models::book::Book,
    },
    layout::{basic_layout::models::BasicLayout, layoutize, models::LayoutEngine},
    pagination::{
        basic_pagination::models::BasicPagination,
        models::{Page, PaginationEngine},
        paginate,
    },
    rendering::models::{AppState, RenderApp, RenderingEngine},
};

pub(crate) struct RatatuiApp<'a> {
    backend: Terminal<CrosstermBackend<Stdout>>,
    state: AppState,

    books: &'a Vec<Book>,
    curr_book_pages: Option<Vec<Page>>,
    curr_book_idx: usize,
    current_page: usize,
}

pub(crate) struct RatatuiEngine;

impl<'a> RenderApp for RatatuiApp<'a> {
    type Error = std::io::Error;

    fn draw(&mut self) -> Result<(), Self::Error> {
        match self.state {
            AppState::Library => self.draw_library(),
            AppState::Reading => self.draw_reader(),
        }
    }

    fn handle_events(&mut self) -> Result<(), Self::Error> {
        if let Event::Key(key) = crossterm::event::read()? {
            match self.state {
                AppState::Library => match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.curr_book_idx + 1 < self.books.len() {
                            self.curr_book_idx += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.curr_book_idx = self.curr_book_idx.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        self.curr_book_pages = Some(self.paginate_current_book()?);
                        self.current_page = 1;
                        self.state = AppState::Reading;
                    }
                    KeyCode::Char('q') => self.shutdown()?,
                    _ => {}
                },
                AppState::Reading => match key.code {
                    KeyCode::Right | KeyCode::Char('l') => {
                        let total = self.curr_book_pages.as_ref().map_or(0, |p| p.len());
                        if self.current_page + 1 < total {
                            self.current_page += 1;
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if self.current_page - 1 > 0 {
                            self.current_page -= 1;
                        }
                    }
                    KeyCode::Backspace => {
                        self.state = AppState::Library;
                        self.current_page = 1;
                    }
                    KeyCode::Char('q') => self.shutdown()?,
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Self::Error> {
        crossterm::terminal::disable_raw_mode()?;
        self.backend.show_cursor()?;
        Ok(())
    }
}

impl<'a> RatatuiApp<'a> {
    fn paginate_current_book(&mut self) -> Result<Vec<Page>, std::io::Error> {
        let book = &self.books[self.curr_book_idx];
        let size = self.backend.size()?;
        let layout = layoutize::<BasicLayout>(book, (size.width - 2) as usize);
        Ok(paginate::<BasicLayout, BasicPagination>(
            layout,
            (size.height - 2) as usize,
        ))
    }

    fn draw_empty(&mut self) -> Result<(), std::io::Error> {
        self.backend.draw(|frame| {
            let paragraph = Paragraph::new("Add some books bro 👍")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Add some books bro 👍 ")
                        .title_bottom(" Add some books bro 👍 "),
                )
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(paragraph, frame.area());
        })?;
        Ok(())
    }

    fn draw_library(&mut self) -> Result<(), std::io::Error> {
        if self.books.is_empty() {
            self.draw_empty()?;
            return Ok(());
        };
        let books = &self.books;
        self.backend.draw(|frame| {
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    ratatui::layout::Constraint::Percentage(40),
                    ratatui::layout::Constraint::Percentage(60),
                ])
                .split(frame.area());

            // book list
            let items: Vec<ratatui::widgets::ListItem> = books
                .iter()
                .enumerate()
                .map(|(idx, b)| {
                    let style = if idx == self.curr_book_idx {
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Yellow)
                            .add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        ratatui::style::Style::default()
                    };
                    ratatui::widgets::ListItem::new(b.get_title()).style(style)
                })
                .collect();
            let list = ratatui::widgets::List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(LIBRARY_LIST_SECTION_NAME)
                    .title_bottom(format!(" Total books: {} ", self.books.len())),
            );
            frame.render_widget(list, chunks[0]);

            // book metadata section
            let selected_book = &books[self.curr_book_idx];
            let paragraph = Paragraph::new(selected_book.get_metadata())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(LIBRARY_METADATA_SECTION_NAME),
                )
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(paragraph, chunks[1]);
        })?;
        Ok(())
    }

    fn draw_reader(&mut self) -> Result<(), std::io::Error> {
        if self.books.is_empty() {
            self.draw_empty()?;
            return Ok(());
        };

        let book = &self.books[self.curr_book_idx];
        let pages = self
            .curr_book_pages
            .as_ref()
            .expect("draw_reader: Pages should be set before setting reading state");

        self.backend.draw(|frame| {
            let page_content = pages[self.current_page].get_content();
            let page_widget_collection: Vec<ratatui::text::Line> = page_content
                .iter()
                .map(|p| ratatui::text::Line::from(p.get_line_content()))
                .collect();

            let paragraph = Paragraph::new(page_widget_collection).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(book.get_title())
                    .title_bottom(format!(
                        "Page: {} / {}",
                        self.current_page,
                        self.curr_book_pages.as_ref().map_or(0, |p| p.len())
                    )),
            );
            frame.render_widget(paragraph, frame.area());
        })?;
        Ok(())
    }
}

impl<'a> RenderingEngine<'a> for RatatuiEngine {
    type OutputRenderer = RatatuiApp<'a>;
    type Error = std::io::Error;

    fn render<L, P>(&mut self, books: &'a Vec<Book>) -> Result<Self::OutputRenderer, Self::Error>
    where
        L: LayoutEngine,
        P: PaginationEngine<L, OutputPages = Vec<Page>>,
    {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
        let backend = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(RatatuiApp {
            backend,
            state: AppState::Library,
            books,
            curr_book_pages: None,
            curr_book_idx: 0,
            current_page: 1,
        })
    }
}
