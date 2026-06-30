use crossterm::event::{Event, KeyCode};
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    widgets::{Block, Borders, ListItem, ListState, Padding, Paragraph},
};

use super::models::RatatuiApp;
use super::models::RatatuiEngine;
use crate::{
    common::{
        constants::{
            LIBRARY_LIST_SECTION_NAME, LIBRARY_METADATA_SECTION_NAME, TUI_HELP_DOC, TUI_PADDING,
        },
        models::{book::Book, settings::Bookmarks},
    },
    layout::{basic_layout::models::BasicLayout, layoutize, models::LayoutEngine},
    pagination::{
        basic_pagination::models::BasicPagination,
        models::{Page, PaginationEngine},
        paginate,
        utils::pages_offset_to_pg_no,
    },
    rendering::{
        models::{AppState, RenderApp, RenderingEngine},
        tui_ratatui::utils::{parse_bottom_title, parse_top_title},
    },
};

impl<'a> RenderApp for RatatuiApp<'a> {
    type Error = std::io::Error;

    fn draw(&mut self) -> Result<(), Self::Error> {
        match self.state {
            AppState::Library => self.draw_library(),
            AppState::Reading => self.draw_reader(),
            AppState::HelpMenu => self.draw_help_menu(),
        }
    }

    fn handle_events(&mut self) -> Result<(), Self::Error> {
        if let Event::Key(key) = crossterm::event::read()? {
            match self.state {
                AppState::Library => match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.list_state.select_next();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.list_state.select_previous();
                    }
                    KeyCode::Enter => {
                        let pages = self.paginate_current_book()?;

                        self.curr_book_lookup = Some(pages_offset_to_pg_no(&pages));
                        self.curr_book_pages = Some(pages);

                        // TODO: make functions for both get and set default bookmarks
                        self.byte_offset = Bookmarks::default()
                            .load_bookmarks()?
                            .get_bookmarks()
                            .get(self.books[self.list_state.selected().unwrap_or(0)].get_id())
                            .map(|b| b.get_offset())
                            .unwrap_or(0); // no bookmark found, start from beginning/
                        self.state = AppState::Reading;
                    }
                    KeyCode::Char('q') => self.shutdown()?,
                    KeyCode::Char('?') => self.state = AppState::HelpMenu,
                    _ => {}
                },
                AppState::Reading => {
                    let pages = self
                        .curr_book_pages
                        .as_ref()
                        .expect("draw_reader: Pages should be set before setting reading state");

                    let page_no = self.get_current_page_no()?;
                    let total_pages = pages.len();

                    match key.code {
                        KeyCode::Right | KeyCode::Char('l') => {
                            if page_no + 1 < total_pages {
                                let next_page: &Page = &pages[page_no + 1];
                                self.byte_offset = next_page.get_start_offset();
                                Bookmarks::default().load_bookmarks()?.set_bookmarks(
                                    self.books[self.list_state.selected().unwrap_or(0)].get_id(),
                                    self.byte_offset,
                                )?;
                            }
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            if page_no > 0 {
                                let prev_page: &Page = &pages[page_no - 1];
                                self.byte_offset = prev_page.get_start_offset();
                                Bookmarks::default().load_bookmarks()?.set_bookmarks(
                                    self.books[self.list_state.selected().unwrap_or(0)].get_id(),
                                    self.byte_offset,
                                )?;
                            }
                        }
                        KeyCode::Backspace => {
                            self.state = AppState::Library;
                            self.byte_offset = 0;
                        }
                        KeyCode::Char('q') => self.shutdown()?,
                        _ => {}
                    }
                }
                AppState::HelpMenu => match key.code {
                    KeyCode::Backspace => self.state = AppState::Library,
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
        self.should_quit = true;
        Ok(())
    }

    fn should_quit(&mut self) -> bool {
        self.should_quit
    }
}

impl<'a> RenderingEngine<'a> for RatatuiEngine {
    type OutputRenderer = RatatuiApp<'a>;
    type Error = std::io::Error;

    fn render<L, P>(&mut self, books: &'a [Book]) -> Result<Self::OutputRenderer, Self::Error>
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
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(RatatuiApp {
            backend,
            state: AppState::Library,
            books,
            curr_book_pages: None,
            list_state,
            should_quit: false,
            byte_offset: 0,
            curr_book_lookup: None,
        })
    }
}

impl<'a> RatatuiApp<'a> {
    fn get_current_page_no(&self) -> Result<usize, std::io::Error> {
        // TODO: Bugfix: if the size of a terminal changes when we have set some byteoffset for a
        // page then it will always default to 0. Need to make size, and dimensions dynamic -
        // something like frame state perhaps. This bug most likely only will be observed when going
        // from large to small dimensions not the other way around (this is not a fucking AI
        // generated comment, shut up)

        let lookup = self
            .curr_book_lookup
            .as_ref()
            .ok_or_else(|| std::io::Error::other("handle_events: Lookup not created"))?;
        let page_no = *lookup.get(&self.byte_offset).unwrap_or(&0usize);
        Ok(page_no)
    }

    fn paginate_current_book(&mut self) -> Result<Vec<Page>, std::io::Error> {
        let book = &self.books[self.list_state.selected().unwrap_or(0)];
        let size = self.backend.size()?;
        let layout =
            layoutize::<BasicLayout>(book, ((size.width - 2) as usize) - (2 * TUI_PADDING));
        Ok(paginate::<BasicLayout, BasicPagination>(
            layout,
            ((size.height - 2) as usize) - (2 * TUI_PADDING),
        ))
    }

    fn draw_empty(&mut self) -> Result<(), std::io::Error> {
        self.backend.draw(|frame| {
            let paragraph = Paragraph::new("Add some books bro 👍")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(parse_top_title("Add some books bro 👍"))
                        .title_bottom(parse_bottom_title("Add some books bro 👍")),
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
            let items: Vec<ratatui::widgets::ListItem> =
                books.iter().map(|b| ListItem::new(b.get_title())).collect();
            let list = ratatui::widgets::List::new(items)
                .highlight_symbol("▶ ")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::uniform(TUI_PADDING as u16))
                        .title(parse_top_title(LIBRARY_LIST_SECTION_NAME))
                        .title_bottom(parse_bottom_title(
                            format!("Total books: {}", self.books.len()).as_str(),
                        )),
                );
            frame.render_stateful_widget(list, chunks[0], &mut self.list_state);

            // book metadata section
            let selected_book = &books[self.list_state.selected().unwrap_or(0)];
            let paragraph = Paragraph::new(selected_book.get_metadata())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::uniform(TUI_PADDING as u16))
                        .title(parse_top_title(LIBRARY_METADATA_SECTION_NAME)),
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

        let book: &Book = &self.books[self.list_state.selected().unwrap_or(0)];
        let pages = self
            .curr_book_pages
            .as_ref()
            .expect("draw_reader: Pages should be set before setting reading state");
        let page_no = self.get_current_page_no()?;
        let current_page: &Page = &pages[page_no];
        let total_pages = pages.len();

        self.backend.draw(|frame| {
            let page_content = current_page.get_content();
            let page_widget_collection: Vec<ratatui::text::Line> = page_content
                .iter()
                .map(|p| ratatui::text::Line::from(p.get_line_content()))
                .collect();

            let paragraph = Paragraph::new(page_widget_collection).block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(TUI_PADDING as u16))
                    .title(parse_top_title(book.get_title()))
                    .title_bottom(parse_bottom_title(
                        format!("Page: {} / {} |", page_no + 1, total_pages).as_str(),
                    )),
            );
            frame.render_widget(paragraph, frame.area());
        })?;
        Ok(())
    }

    fn draw_help_menu(&mut self) -> Result<(), std::io::Error> {
        self.backend.draw(|frame| {
            let help_block = Paragraph::new(TUI_HELP_DOC).block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(TUI_PADDING as u16))
                    .title(parse_top_title("Welcome to the helpdesk")),
            );
            frame.render_widget(help_block, frame.area());
        })?;
        Ok(())
    }
}
