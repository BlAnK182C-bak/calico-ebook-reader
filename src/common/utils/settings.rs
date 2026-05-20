use std::{collections::HashMap, fs};

use crate::common::{
    constants::{BOOKMAP_FILE_PATH, SETTINGS_FILE_PATH},
    models::{
        filetypes::BookFileTypes,
        settings::{BookMap, Settings},
    },
};

pub(crate) fn scan_sources_for_books()
-> Result<HashMap<String, BookFileTypes>, Box<dyn std::error::Error>> {
    let mut all_books: HashMap<String, BookFileTypes> = if BOOKMAP_FILE_PATH.exists() {
        let bookmap_content = fs::read_to_string(BOOKMAP_FILE_PATH.to_path_buf())?;
        if bookmap_content.trim().is_empty() {
            HashMap::new()
        } else {
            let res: Vec<BookMap> = serde_json::from_str(&bookmap_content)?;
            res.into_iter()
                .map(|item| (String::from(item.get_filepath()), item.get_filetype()))
                .collect()
        }
    } else {
        HashMap::new()
    };

    // scan for books in source settings
    let settings_file_content = fs::read_to_string(SETTINGS_FILE_PATH.to_path_buf())?;
    let settings: Settings = toml::from_str(&settings_file_content)?;
    for source in settings.get_get_source_paths().iter() {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let stringified_path = path.to_string_lossy().to_string();

                if all_books.contains_key(&stringified_path) {
                    println!(
                        "scan_sources_for_books: Skipping already extracted: {}",
                        stringified_path
                    );
                    continue;
                }
                let Some(extension) = path
                    .extension()
                    .and_then(|e| e.to_str().filter(|e| !e.is_empty()))
                else {
                    println!(
                        "scan_sources_for_books: Warning: Found: {} without a file extension.",
                        stringified_path
                    );
                    continue;
                };
                let file_types = BookFileTypes::new(extension);
                all_books.insert(stringified_path, file_types);
            }
        }
    }
    Ok(all_books)
}
