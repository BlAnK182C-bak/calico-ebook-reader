pub(super) mod extractors;
pub(crate) mod models;

use models::RawEpub;

pub(crate) fn epub_parse(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_epub: RawEpub = RawEpub::new(file_path);
    new_epub.extract_epub_file()?;
    new_epub.validate()?;
    new_epub.init()?;
    println!("{:#?}", new_epub);
    let m = new_epub.extract_epub_metadata()?;
    println!("{:#?}", m);
    Ok(())
}
