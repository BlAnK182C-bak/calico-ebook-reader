use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::constants::BOOKMARKS_FILE_PATH;

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Bookmarks {
    bookmarks: HashMap<String, BookBookmark>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BookBookmark {
    offset: usize,
    last_read: usize,
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
        book_id: &String,
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
