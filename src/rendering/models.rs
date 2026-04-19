use crate::{
    common::models::book::Book, layout::models::LayoutEngine, pagination::models::PaginationEngine,
};

pub(crate) trait RenderApp {
    type Error;
    fn draw(&mut self, book_name: &str) -> Result<(), Self::Error>;
    fn handle_events(&mut self) -> Result<(), Self::Error>;
    fn run(&mut self, book_name: &str) -> Result<(), Self::Error> {
        loop {
            self.draw(book_name)?;
            self.handle_events()?;
        }
    }
    fn shutdown(&mut self) -> Result<(), Self::Error>;
}

pub(crate) trait RenderingEngine<L: LayoutEngine, P: PaginationEngine<L>> {
    type OutputRenderer: RenderApp;
    type Error;
    fn render(&mut self, book: &Book) -> Result<Self::OutputRenderer, Self::Error>;
}
