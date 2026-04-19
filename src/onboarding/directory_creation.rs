use std::fs;

use crate::common::constants::{
    self, APPLICATION_DATA_PATH, BOOKS_DIR_PATH, CONFIG_DIR_PATH, EPUB_DIR_PATH,
};

// TODO: Optimization: Make a single helper function for creating directories and call it in the individual functions

/// Directory of the entire application:
/// This is the root of the entire app that contains all information
fn create_application_directory() {
    match fs::exists(APPLICATION_DATA_PATH.as_path()) {
        Ok(file_exists) => {
            if file_exists {
                println!("create_application_directory: Application directory already exists.");
            } else {
                fs::create_dir(constants::APPLICATION_DATA_PATH.as_path())
                    .expect("Failed to create application directory.");
                println!("Created application directory successfully.")
            }
        }
        Err(err) => {
            panic!(
                "An error occurred while creating application directory: {}",
                err
            )
        }
    }
}

///Books Directory
fn create_books_directory() {
    match fs::exists(BOOKS_DIR_PATH.as_path()) {
        Ok(file_exists) => {
            if file_exists {
                println!("create_books_directory: books directory already exists.");
            } else {
                fs::create_dir(constants::BOOKS_DIR_PATH.as_path())
                    .expect("Failed to create books directory.");

                println!("Created books directory successfully.")
            }
        }
        Err(err) => {
            panic!("An error occurred while creating books directory: {}", err)
        }
    }
}

///Epub Books Directory
fn create_epubs_directory() {
    match fs::exists(EPUB_DIR_PATH.as_path()) {
        Ok(file_exists) => {
            if file_exists {
                println!("create_epubs_directory: epubs directory already exists.");
            } else {
                fs::create_dir(constants::EPUB_DIR_PATH.as_path())
                    .expect("Failed to create epubs directory.");
                println!("Created epubs directory successfully.")
            }
        }
        Err(err) => {
            panic!("An error occurred while creating epubs directory: {}", err)
        }
    }
}

// configs directory
fn create_configs_directory() {
    println!("{}", CONFIG_DIR_PATH.to_str().unwrap());
    match fs::exists(CONFIG_DIR_PATH.as_path()) {
        Ok(file_exists) => {
            if file_exists {
                println!("create_configs_directory: configs directory already exists.");
            } else {
                fs::create_dir(constants::CONFIG_DIR_PATH.as_path())
                    .expect("Failed to create configs directory.");
                println!("Created configs directory successfully.")
            }
        }
        Err(err) => {
            panic!(
                "An error occurred while creating configs directory: {}",
                err
            )
        }
    }
}

pub(super) fn create_all_directories() {
    println!("Starting creation of all directories...");
    create_application_directory();
    create_books_directory();
    create_epubs_directory();
    create_configs_directory();
}
