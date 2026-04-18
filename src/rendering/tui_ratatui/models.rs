use std::io::Stdout;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    common::models::book::Book,
    layout::{basic_layout::models::BasicLayout, layoutize, models::LayoutEngine},
    pagination::{
        models::{Page, PaginationEngine},
        paginate,
    },
    rendering::models::{RenderApp, RenderingEngine},
};

pub(crate) struct RatatuiApp {
    backend: Terminal<CrosstermBackend<Stdout>>,
    pages: Vec<Page>,
    current_page: usize,
}

pub(crate) struct RatatuiEngine;

impl RenderApp for RatatuiApp {
    type Error = std::io::Error;
    fn draw(&mut self, book_name: &str) -> Result<(), Self::Error> {
        let page = &self.pages[self.current_page];
        let drawable_lines: Vec<ratatui::text::Line> = page
            .get_content()
            .iter()
            .map(|l| ratatui::text::Line::from(Span::raw(l.get_line_content().to_string())))
            .collect();

        self.backend.draw(|frame| {
            let paragraph = Paragraph::new(drawable_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", book_name))
                    .title_bottom(format!(
                        " Page: {} of {} ",
                        self.current_page,
                        self.pages.len()
                    )),
            );

            frame.render_widget(paragraph, frame.area());
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), Self::Error> {
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Right | KeyCode::Char('l') => {
                    if self.current_page + 1 < self.pages.len() {
                        self.current_page += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if self.current_page - 1 > 0 {
                        self.current_page -= 1;
                    }
                }
                KeyCode::Char('q') => self.shutdown()?,
                _ => {}
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

impl<L, P> RenderingEngine<L, P> for RatatuiEngine
where
    L: LayoutEngine,
    P: PaginationEngine<L, OutputPages = Vec<Page>>,
{
    type OutputRenderer = RatatuiApp;
    type Error = std::io::Error;
    fn render(&mut self, book: Book) -> Result<Self::OutputRenderer, Self::Error> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
        let backend = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        let backend_size = backend.size()?;
        let layout = layoutize::<L>(book, backend_size.width as usize);
        let pages = paginate::<L, P>(layout, backend_size.height as usize);

        Ok(RatatuiApp {
            backend,
            pages,
            current_page: 0,
        })
    }
}
