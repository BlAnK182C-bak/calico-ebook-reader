use crate::{
    common::models::book::Book,
    layout::models::LayoutEngine,
    pagination::models::{Page, PaginationEngine},
};

pub(crate) enum AppState {
    Library,
    Reading,
}

pub(crate) trait RenderApp {
    type Error;
    fn draw(&mut self) -> Result<(), Self::Error>;
    fn handle_events(&mut self) -> Result<(), Self::Error>;
    fn should_quit(&mut self) -> bool;
    fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            self.draw()?;
            self.handle_events()?;
            if self.should_quit() {
                break;
            }
        }
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub(crate) trait RenderingEngine<'a> {
    type OutputRenderer: RenderApp;
    type Error;
    fn render<L, P>(&mut self, books: &'a Vec<Book>) -> Result<Self::OutputRenderer, Self::Error>
    where
        L: LayoutEngine,
        P: PaginationEngine<L, OutputPages = Vec<Page>>;
}
