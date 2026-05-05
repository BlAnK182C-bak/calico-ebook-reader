use std::fs;

use crate::common::constants::{
    APPLICATION_DATA_PATH, BOOKS_DIR_PATH, CONFIG_DIR_PATH, EPUB_DIR_PATH,
};

fn create_dir(dir_path: &str, dir_name: &str) -> Result<(), std::io::Error> {
    match fs::exists(dir_path) {
        Ok(file_exists) => {
            if file_exists {
                println!("create_dir: {} directory already exists.", dir_name);
                Ok(())
            } else {
                fs::create_dir(dir_path)
                    .expect(format!("Failed to create {} directory.", dir_name).as_str());
                println!("Created {} directory successfully.", dir_name);
                Ok(())
            }
        }
        Err(err) => {
            panic!(
                "An error occurred while creating {} directory: {}",
                dir_name, err
            )
        }
    }
}

// Directory of the entire application:
// This is the root of the entire app that contains all information
fn create_application_directory() -> Result<(), std::io::Error> {
    create_dir(
        APPLICATION_DATA_PATH.to_str().ok_or_else(|| {
            std::io::Error::other(
                "create_application_directory: Failed to convert application data path to string",
            )
        })?,
        "Applications",
    )?;
    Ok(())
}

// Books Directory
fn create_books_directory() -> Result<(), std::io::Error> {
    create_dir(
        BOOKS_DIR_PATH.to_str().ok_or_else(|| {
            std::io::Error::other(
                "create_application_directory: Failed to convert application data path to string",
            )
        })?,
        "Applications",
    )?;
    Ok(())
}

// Epub Books Directory
fn create_epubs_directory() -> Result<(), std::io::Error> {
    create_dir(
        EPUB_DIR_PATH.to_str().ok_or_else(|| {
            std::io::Error::other(
                "create_application_directory: Failed to convert application data path to string",
            )
        })?,
        "Applications",
    )?;
    Ok(())
}

// configs directory
fn create_configs_directory() -> Result<(), std::io::Error> {
    create_dir(
        CONFIG_DIR_PATH.to_str().ok_or_else(|| {
            std::io::Error::other(
                "create_application_directory: Failed to convert application data path to string",
            )
        })?,
        "Applications",
    )?;
    Ok(())
}

pub(super) fn create_all_directories() -> Result<(), std::io::Error> {
    println!("Starting creation of all directories...");
    create_application_directory()?;
    create_books_directory()?;
    create_epubs_directory()?;
    create_configs_directory()?;
    Ok(())
}
