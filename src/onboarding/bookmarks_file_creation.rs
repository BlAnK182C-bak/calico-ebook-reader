use std::fs::File;

use crate::common::constants::BOOKMARKS_FILE_PATH;

pub(super) fn create_bookmarks_file() -> Result<(), std::io::Error> {
    if BOOKMARKS_FILE_PATH.exists() {
        println!("create_settings_file: Settings file already exists. Skipping creation");
        Ok(())
    } else {
        File::create(BOOKMARKS_FILE_PATH.to_path_buf())?;
        Ok(())
    }
}

pub(super) fn bookmarks_pipeline() -> Result<(), std::io::Error> {
    create_bookmarks_file()?;
    Ok(())
}
