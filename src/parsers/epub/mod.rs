pub(crate) mod models;
pub(super) mod utils;

use crate::parsers::models::{Book, BookFileTypes, ParserEngine};
use crate::parsers::utils::get_file_type_from_path;
use models::RawEpub;

impl ParserEngine for RawEpub {
    fn parse(&mut self) -> Result<Book, Box<dyn std::error::Error>> {
        self.extract_epub_file()?;
        self.validate()?;
        self.init()?;
        let new_epub_metadata = self.extract_epub_metadata()?;
        let new_epub_sections = self.extract_epub_content()?;
        let new_epub_file_type = get_file_type_from_path(self.get_file_path())?;

        let book = Book::new(
            new_epub_metadata,
            BookFileTypes::new(new_epub_file_type),
            new_epub_sections,
        );

        Ok(book)
    }
}
