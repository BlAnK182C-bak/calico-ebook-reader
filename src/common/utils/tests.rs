use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

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
