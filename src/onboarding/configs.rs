use std::fs::{self, File};

use crate::common::{
    constants::{BOOKMAP_FILE_PATH, BOOKMARKS_FILE_PATH, SETTINGS_FILE_PATH},
    models::settings::{Settings, SourceSettings},
};

fn create_settings_file() -> Result<(), std::io::Error> {
    if SETTINGS_FILE_PATH.exists() {
        println!("create_settings_file: Settings file already exists. Skipping creation");
        Ok(())
    } else {
        File::create(SETTINGS_FILE_PATH.to_path_buf())?;
        Ok(())
    }
}

fn create_source_setting() -> Result<(), std::io::Error> {
    let home = std::env::var("HOME").map_err(std::io::Error::other)?;
    let ss = SourceSettings::new(vec![format!("{}/Documents/", home)]);
    let default_settings = &Settings::new(ss);

    let existing_content = fs::read_to_string(SETTINGS_FILE_PATH.to_path_buf())?;
    if !existing_content.trim().is_empty()
        && let Ok(settings) = toml::from_str::<Settings>(&existing_content)
        && !settings.get_get_source_paths().is_empty()
    {
        println!("create_source_setting: Sources already exist, skipping addition of defaults");
        return Ok(());
    }

    let contents = toml::to_string_pretty(default_settings).expect("Failed to serialize config");
    fs::write(SETTINGS_FILE_PATH.to_path_buf(), contents)?;
    Ok(())
}

pub(super) fn create_bookmarks_file() -> Result<(), std::io::Error> {
    if BOOKMARKS_FILE_PATH.exists() {
        println!("create_bookmarks_file: Bookmarks file already exists. Skipping creation");
        Ok(())
    } else {
        File::create(BOOKMARKS_FILE_PATH.to_path_buf())?;
        Ok(())
    }
}

pub(super) fn create_bookmap_file() -> Result<(), std::io::Error> {
    if BOOKMAP_FILE_PATH.exists() {
        println!("create_bookmap_file: Bookmap file already exists. Skipping creation");
        Ok(())
    } else {
        File::create(BOOKMAP_FILE_PATH.to_path_buf())?;
        Ok(())
    }
}

pub(super) fn configs_pipeline() -> Result<(), std::io::Error> {
    println!("Running the configs onboarding pipeline...");
    create_settings_file()?;
    create_source_setting()?;
    create_bookmarks_file()?;
    create_bookmap_file()?;
    Ok(())
}
