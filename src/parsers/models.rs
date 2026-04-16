use crate::common::models::book::Book;

pub(crate) trait ParserEngine {
    fn parse(&mut self) -> Result<Book, Box<dyn std::error::Error>>;
}
