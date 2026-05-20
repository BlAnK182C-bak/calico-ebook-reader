use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::{constants::BOOKMARKS_FILE_PATH, models::filetypes::BookFileTypes};
#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    sources: SourceSettings,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SourceSettings {
    source_paths: Vec<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Bookmarks {
    bookmarks: HashMap<String, BookBookmark>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BookBookmark {
    offset: usize,
    last_read: usize,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BookMap<'a> {
    uuid: Uuid,
    filename: &'a str,
    filepath: &'a str,
    filetype: &'a str,
}

impl Settings {
    pub(crate) fn new(source_settings: SourceSettings) -> Self {
        Self {
            sources: source_settings,
        }
    }
    pub(crate) fn get_get_source_paths(&self) -> &Vec<String> {
        &self.sources.source_paths
    }
}

impl SourceSettings {
    pub(crate) fn new(source_paths: Vec<String>) -> Self {
        Self { source_paths }
    }
}

impl Bookmarks {
    pub(crate) fn load_bookmarks(&self) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(BOOKMARKS_FILE_PATH.to_path_buf())?;
        if contents.is_empty() {
            Ok(Bookmarks::default())
        } else {
            toml::from_str(&contents).map_err(|e| std::io::Error::other(e))
        }
    }

    pub(crate) fn get_bookmarks(&self) -> &HashMap<String, BookBookmark> {
        &self.bookmarks
    }

    pub(crate) fn set_bookmarks(
        &mut self,
        book_id: &str,
        offset: usize,
    ) -> Result<(), std::io::Error> {
        self.bookmarks.insert(
            book_id.into(),
            BookBookmark {
                offset,
                last_read: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as usize,
            },
        );

        let contents = toml::to_string(self).map_err(|e| std::io::Error::other(e))?;
        std::fs::write(BOOKMARKS_FILE_PATH.to_path_buf(), contents)
    }
}

impl BookBookmark {
    pub(crate) fn get_offset(&self) -> usize {
        self.offset
    }
}

impl<'a> BookMap<'a> {
    pub(crate) fn new(uuid: Uuid, filename: &'a str, filepath: &'a str, filetype: &'a str) -> Self {
        Self {
            uuid,
            filename,
            filepath,
            filetype,
        }
    }
    pub(crate) fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub(crate) fn get_filename(&self) -> &str {
        &self.filename
    }

    pub(crate) fn get_filepath(&self) -> &str {
        self.filepath
    }

    pub(crate) fn get_filetype(&self) -> BookFileTypes {
        BookFileTypes::new(self.filetype)
    }
}
