use directories::ProjectDirs;
use std::path::PathBuf;
use std::sync::LazyLock;

// general constants
pub(crate) const APPLICATION_NAME: &str = "calico_ebook_reader";
pub(crate) const APPLICATION_DOMAIN: &str = "com";
pub(crate) const APPLICATION_AUTHOR: &str = "Abhinav Kumar Singh";

// book related constants
pub(crate) const BOOKS_DIR_NAME: &str = "CER_Books";
pub(crate) const EPUB_DIR_NAME: &str = "CER_Epubs";

pub(crate) const EPUB_MIMETYPE: &str = "application/epub+zip";
pub(crate) const EPUB_ENTRY_POINT: &str = "META-INF/container.xml";

// file path related constants
fn extract_project_dir(
    qualifier: &str,
    organization: &str,
    application: &str,
) -> Option<ProjectDirs> {
    ProjectDirs::from(qualifier, organization, application)
}

// TODO: Optimization: Make BOOKS_DIR_PATH and EPUB_DIR_PATH simply APPLICATION_DATA_PATH.join()

pub(crate) static APPLICATION_DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    extract_project_dir(APPLICATION_DOMAIN, APPLICATION_AUTHOR, APPLICATION_NAME)
        .expect("BOOKS_DIR_PATH: There was an error creating/finding the project directories")
        .data_dir()
        .to_path_buf()
});

pub(crate) static BOOKS_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path: PathBuf =
        extract_project_dir(APPLICATION_DOMAIN, APPLICATION_AUTHOR, APPLICATION_NAME)
            .expect("BOOKS_DIR_PATH: There was an error creating/finding the project directories")
            .data_dir()
            .to_path_buf();
    path.push(PathBuf::from(BOOKS_DIR_NAME));
    path
});

pub(crate) static EPUB_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path: PathBuf =
        extract_project_dir(APPLICATION_DOMAIN, APPLICATION_AUTHOR, APPLICATION_NAME)
            .expect("EPUB_DIR_PATH:There was an error creating/finding  the project directories")
            .data_dir()
            .to_path_buf();
    path.push(PathBuf::from(BOOKS_DIR_NAME));
    path.push(PathBuf::from(EPUB_DIR_NAME));
    path
});
