use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

use crate::common::constants::{BOOKMAP_FILE_PATH, EPUB_DIR_PATH};
use crate::common::models::filetypes::BookFileTypes;
use crate::common::models::settings::BookMap;

pub(crate) fn get_book_folder_path(
    file_type: BookFileTypes,
    file_name: &str,
) -> Result<PathBuf, std::io::Error> {
    match file_name.split(".").next() {
        Some(file_name_without_extension) => match file_type {
            BookFileTypes::EpubFileType => Ok(EPUB_DIR_PATH.join(Path::new(
                &get_book_uuid(
                    "epub",
                    file_name_without_extension,
                    EPUB_DIR_PATH.to_path_buf(),
                )?
                .to_string(),
            ))),
            BookFileTypes::UnknownFileType => Err(std::io::Error::other(
                "get_book_folder_name: Unknown file types",
            )),
        },
        None => Err(std::io::Error::other(
            "get_book_folder_name: This file has a blank name",
        )),
    }
}

static BOOKMAP_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub(crate) fn get_book_uuid(
    filetype: &str,
    file_name: &str,
    parent_file_path: PathBuf,
) -> Result<Uuid, std::io::Error> {
    let lock = BOOKMAP_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();

    let bookmap_exists = fs::exists(BOOKMAP_FILE_PATH.to_path_buf())?;
    if bookmap_exists {
        let file_contents = fs::read_to_string(BOOKMAP_FILE_PATH.to_path_buf())?;
        let mut book_map: Vec<BookMap> = serde_json::from_str(&file_contents).unwrap_or(vec![]);

        // TODO: lookup can change from O(n) to O(1) if we use hashmaps - whether we do that or not,
        // we can optimize existing lookup much better
        for bookmap_field in book_map.iter() {
            if bookmap_field.get_filename() == file_name {
                return Ok(bookmap_field.get_uuid());
            }
        }

        let new_book_uuid = Uuid::new_v4();

        let file_path = parent_file_path.join(new_book_uuid.to_string());

        book_map.push(BookMap::new(
            new_book_uuid,
            file_name,
            file_path.to_str().unwrap(),
            filetype,
        ));

        let tmp = format!("{}.tmp", BOOKMAP_FILE_PATH.to_string_lossy());
        let file = fs::File::create(&tmp)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &book_map).map_err(|e| std::io::Error::other(e))?;
        fs::rename(&tmp, BOOKMAP_FILE_PATH.as_path())?;

        Ok(new_book_uuid)
    } else {
        Err(std::io::Error::other(
            "get_book_uuid: bookmap file doesn't exist",
        ))
    }
}

pub(crate) fn get_file_name_from_path(file_path: &str) -> Result<&str, &str> {
    match file_path.split("/").last() {
        Some(file_name) => Ok(file_name),
        None => Err("get_file_name_from_path: Couldn't find a file of this file path"),
    }
}
