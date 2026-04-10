use std::path::{Path, PathBuf};

use super::models::BookFileTypes;
use crate::misc::constants::EPUB_DIR_PATH;

// TODO: Bugfix: duplicate file names (from two different folders let's say) - use uuid to generate
// a unique uuid for each book
pub(crate) fn get_book_folder_name(
    file_type: BookFileTypes,
    file_name: &str,
) -> Result<PathBuf, &str> {
    match file_name.split(".").next() {
        Some(file_name_without_extension) => match file_type {
            BookFileTypes::EpubFileType => {
                Ok(EPUB_DIR_PATH.join(Path::new(file_name_without_extension)))
            }
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
