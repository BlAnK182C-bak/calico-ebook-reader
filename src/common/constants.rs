use std::path::PathBuf;
use std::sync::LazyLock;

use super::utils::project_dir::extract_project_dir;

// general constants
pub(crate) const APPLICATION_NAME: &str = "calico_ebook_reader";
pub(crate) const APPLICATION_DOMAIN: &str = "com";
pub(crate) const APPLICATION_AUTHOR: &str = "Abhinav Kumar Singh";

// settings based constants
pub(crate) const SETTINGS_FILENAME: &str = "settings.toml";

// reader constants
pub(crate) const LIBRARY_LIST_SECTION_NAME: &str = " Library ";
pub(crate) const LIBRARY_METADATA_SECTION_NAME: &str = " Metadata ";

// book related constants
pub(crate) const BOOKS_DIR_NAME: &str = "CER_Books";
pub(crate) const EPUB_DIR_NAME: &str = "CER_Epubs";

pub(crate) const EPUB_MIMETYPE: &str = "application/epub+zip";
pub(crate) const EPUB_ENTRY_POINT: &str = "META-INF/container.xml";

// file path related constants
pub(crate) static APPLICATION_DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    extract_project_dir(APPLICATION_DOMAIN, APPLICATION_AUTHOR, APPLICATION_NAME)
        .expect("BOOKS_DIR_PATH: There was an error creating/finding the project directories")
        .data_dir()
        .to_path_buf()
});

pub(crate) static BOOKS_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path: PathBuf = APPLICATION_DATA_PATH.to_path_buf();
    path.push(PathBuf::from(BOOKS_DIR_NAME));
    path
});

pub(crate) static EPUB_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path: PathBuf = APPLICATION_DATA_PATH.to_path_buf();
    path.push(PathBuf::from(EPUB_DIR_NAME));
    path
});

pub(crate) static CONFIG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    extract_project_dir(APPLICATION_DOMAIN, APPLICATION_AUTHOR, APPLICATION_NAME)
        .expect("BOOKS_DIR_PATH: There was an error creating/finding the project directories")
        .config_dir()
        .to_path_buf()
});

pub(crate) static SETTINGS_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR_PATH.to_path_buf();
    path.push(SETTINGS_FILENAME);
    path
});
