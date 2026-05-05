use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::common::constants::EPUB_DIR_PATH;
use crate::common::models::filetypes::BookFileTypes;

pub(crate) fn get_book_folder_name(
    file_type: BookFileTypes,
    file_name: &str,
) -> Result<PathBuf, &str> {
    match file_name.split(".").next() {
        Some(file_name_without_extension) => match file_type {
            BookFileTypes::EpubFileType => Ok(EPUB_DIR_PATH.join(Path::new(
                format!("{}-{}", file_name_without_extension, Uuid::new_v4()).as_str(),
            ))),
            BookFileTypes::UnknownFileType => Err("get_book_folder_name: Unknown file types"),
        },
        None => Err("get_book_folder_name: This file has a blank name"),
    }
}

pub(crate) fn get_file_name_from_path(file_path: &str) -> Result<&str, &str> {
    match file_path.split("/").last() {
        Some(file_name) => Ok(file_name),
        None => Err("get_file_name_from_path: Couldn't find a file of this file path"),
    }
}

pub(crate) fn get_file_type_from_path(file_path: &str) -> Result<&str, &str> {
    match file_path.split("/").last() {
        Some(file_name) => match file_name.split(".").last() {
            Some(file_type) => Ok(file_type),
            None => Err("get_file_type_from_path: This file doesn't seem to have a type."),
        },
        None => Err("get_file_type_from_path: Coduln't find a file of this file path"),
    }
}
