use std::{collections::HashMap, fs};

use crate::common::{
    constants::SETTINGS_FILE_PATH,
    models::{filetypes::BookFileTypes, settings::Settings},
};

pub(crate) fn scan_sources_for_books()
-> Result<HashMap<String, BookFileTypes>, Box<dyn std::error::Error>> {
    let settings_file_content = fs::read_to_string(SETTINGS_FILE_PATH.to_path_buf())?;
    let settings: Settings = toml::from_str(&settings_file_content)?;
    let mut all_books: HashMap<String, BookFileTypes> = HashMap::new();
    for source in settings.get_get_source_paths().iter() {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension().and_then(|e| e.to_str()).ok_or_else(|| {
                    std::io::Error::other("scan_sources_for_books: Error parsing the OsStr to str")
                })?;
                let file_types = BookFileTypes::new(extension);
                all_books.insert(path.to_string_lossy().to_string(), file_types);
            }
        }
    }
    Ok(all_books)
}
