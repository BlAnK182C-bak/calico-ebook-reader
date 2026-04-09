pub(super) mod content_extractors;
pub(crate) mod models;

use models::RawEpub;

use crate::misc::{
    models::{Book, BookFileTypes},
    utils::get_file_type_from_path,
};

pub(crate) fn epub_parse(file_path: &str) -> Result<Book, Box<dyn std::error::Error>> {
    let mut new_epub: RawEpub = RawEpub::new(file_path);
    new_epub.extract_epub_file()?;
    new_epub.validate()?;
    new_epub.init()?;
    let new_epub_metadata = new_epub.extract_epub_metadata()?;
    let new_epub_sections = new_epub.extract_epub_content()?;
    let new_epub_file_type = get_file_type_from_path(file_path)?;

    let book = Book::new(
        new_epub_metadata,
        BookFileTypes::new(new_epub_file_type),
        new_epub_sections,
    );

    Ok(book)
}
