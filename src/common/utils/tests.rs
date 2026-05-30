use crate::common::constants::{EPUB_ENTRY_POINT, EPUB_MIMETYPE};
use crate::pagination::models::Page;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::common::models::book::{Book, BookMetadata, BookSection};

pub(crate) fn write_bookmap(tempdir: &TempDir, bookmap_content: &str) -> PathBuf {
    let path = tempdir.path().join("bookmap.json");
    fs::write(&path, bookmap_content).unwrap();
    path
}

pub(crate) fn make_ebooks_dir(tempdir: &TempDir) -> PathBuf {
    let path = tempdir.path().join("epubs/");
    fs::create_dir(&path).unwrap();
    path
}

pub(crate) fn write_settings(tempdir: &TempDir, source_paths: &[&str]) -> PathBuf {
    let path = tempdir.path().join("settings.toml");
    let paths_toml = source_paths
        .iter()
        .map(|p| format!("\"{}\"", p))
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(&path, format!("[sources]\nsource_paths = [{}]", paths_toml)).unwrap();
    path
}

pub(crate) fn create_valid_epub_structure() -> Result<tempfile::TempDir, std::io::Error> {
    let dir = TempDir::new()?;

    fs::write(dir.path().join("mimetype"), EPUB_MIMETYPE)?;
    let container_path = dir.path().join(EPUB_ENTRY_POINT);
    if let Some(parent) = container_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        &container_path,
        r#"<?xml version="1.0"?>
<container>
    <rootfiles>
        <rootfile full-path="OPS/content.opf" />
    </rootfiles>
</container>"#,
    )?;
    let ops_dir = dir.path().join("OPS");
    fs::create_dir_all(&ops_dir)?;
    fs::write(ops_dir.join("content.opf"), "<package></package>")?;

    Ok(dir)
}

pub(crate) fn create_test_epub_zip_file(
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    zip.start_file("mimetype", options)?;
    zip.write_all(b"application/epub+zip")?;
    zip.start_file("META-INF/container.xml", options)?;
    zip.write_all(
        br#"<?xml version="1.0"?>
<container>
    <rootfiles>
        <rootfile full-path="OPS/content.opf"/>
    </rootfiles>
</container>"#,
    )?;
    zip.start_file("OPS/content.opf", options)?;
    zip.write_all(b"<package></package>")?;
    zip.finish()?;

    Ok(())
}
pub(crate) fn create_page(start_offset: usize) -> Page {
    Page::new(vec![], start_offset, start_offset + 100)
}

pub(crate) fn create_book_section(id: &str, content: &str) -> BookSection {
    BookSection::new(id.to_string(), content.to_string())
}

pub(crate) fn create_book(sections: Vec<BookSection>) -> Book {
    let metadata = BookMetadata::new(
        "Test Book".to_string(),
        Some("Test Author".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    Book::new(metadata, sections)
}
