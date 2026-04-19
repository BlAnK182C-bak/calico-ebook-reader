use std::fs::{self, File};

use crate::common::{
    constants::SETTINGS_FILE_PATH,
    models::settings::{Settings, SourceSettings},
};

fn create_settings_file() -> Result<(), std::io::Error> {
    File::create(SETTINGS_FILE_PATH.to_path_buf())?;
    Ok(())
}

fn create_source_setting(settings: &Settings) -> Result<(), std::io::Error> {
    let contents = toml::to_string_pretty(settings).expect("Failed to serialize config");
    fs::write(SETTINGS_FILE_PATH.to_path_buf(), contents)?;
    Ok(())
}

pub(super) fn settings_pipeline() -> Result<(), std::io::Error> {
    println!("Running the settings onboarding pipeline...");
    create_settings_file()?;
    let home = std::env::var("HOME").map_err(std::io::Error::other)?;
    let ss = SourceSettings::new(vec![format!("{}/Downloads/", home)]);
    create_source_setting(&Settings::new(ss))?;
    Ok(())
}
