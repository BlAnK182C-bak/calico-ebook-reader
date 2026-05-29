use std::{collections::HashMap, fs, path::PathBuf};

use crate::common::models::{
    filetypes::BookFileTypes,
    settings::{BookMap, Settings},
};

pub(crate) fn scan_sources_for_books(
    bookmap_file_path: PathBuf,
    settings_file_path: PathBuf,
) -> Result<HashMap<String, BookFileTypes>, Box<dyn std::error::Error>> {
    let mut all_books: HashMap<String, BookFileTypes> = if bookmap_file_path.exists() {
        let file_contents = fs::read_to_string(bookmap_file_path.to_path_buf())?;
        if file_contents.trim().is_empty() {
            HashMap::new()
        } else {
            let bookmap: Vec<BookMap> = serde_json::from_str(&file_contents)?;
            bookmap
                .into_iter()
                .map(|map| (String::from(map.get_filepath()), map.get_filetype()))
                .collect()
        }
    } else {
        HashMap::new()
    };

    // scan for books in source settings
    let settings_file_content = fs::read_to_string(settings_file_path.to_path_buf())?;
    let settings: Settings = toml::from_str(&settings_file_content)?;
    if settings.get_get_source_paths().len() == 0 {
        println!(
            "scan_sources_for_books: Warning: no sources, skipping creation of all_books entirely"
        );
    } else {
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
    }
    Ok(all_books)
}

#[cfg(test)]
mod source_scanning_tests {
    use crate::common::models::filetypes::BookFileTypes;
    use crate::common::utils::tests::{write_bookmap, write_settings};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn existing_sources_no_exsting_bookmap() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;
        let source_dir = tempdir.path().join("books");
        fs::create_dir(&source_dir)?;
        let epub_path_1 = source_dir.join("new_book_1.epub");
        fs::write(&epub_path_1, b"")?;
        let settings_path = write_settings(&tempdir, &[source_dir.to_str().unwrap()]);
        let bookmap_path = tempdir.path().join("bookmap.json");

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        let key_1 = epub_path_1.to_string_lossy().to_string();

        assert!(res.contains_key(&key_1));
        assert!(matches!(res[&key_1], BookFileTypes::EpubFileType));

        Ok(())
    }

    #[test]
    fn bookmap_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;
        let settings_path = tempdir.path().join("settings.toml");
        write_settings(&tempdir, &[""]);
        let bookmap_path = tempdir.path().join("bookmap.json");

        let err = super::scan_sources_for_books(bookmap_path, settings_path).unwrap_err();
        assert!(
            err.to_string().contains("No such file or directory"),
            "unexpected error: {err}"
        );
        Ok(())
    }

    #[test]
    fn settings_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;
        let settings_path = tempdir.path().join("settings.toml");
        let bookmap_path = tempdir.path().join("bookmap.json");
        write_bookmap(&tempdir, "");

        let err = super::scan_sources_for_books(bookmap_path, settings_path).unwrap_err();
        assert!(
            err.to_string().contains("No such file or directory"),
            "unexpected error: {err}"
        );

        Ok(())
    }

    #[test]
    fn empty_source_file_and_bookmap() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;
        let settings_path = tempdir.path().join("settings.toml");
        write_settings(&tempdir, &[]);
        let bookmap_path = tempdir.path().join("bookmap.json");
        write_bookmap(&tempdir, "");

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        assert_eq!(res.len(), 0);
        Ok(())
    }

    #[test]
    fn multiple_sources_and_books() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;
        let source_dir_names = ["book_source_1", "book_source_2"];
        let book_names = ["new_book_1.epub", "new_book_2.epub", "new_book_3.epub"];

        let mut all_epub_paths: Vec<String> = Vec::new();
        let mut source_dir_paths: Vec<String> = Vec::new();

        for dir_name in source_dir_names {
            let source_dir = tempdir.path().join(dir_name);
            fs::create_dir(&source_dir)?;
            source_dir_paths.push(source_dir.to_string_lossy().into_owned());
            for book_name in book_names {
                let epub_path = source_dir.join(book_name);
                fs::write(&epub_path, b"")?;
                all_epub_paths.push(epub_path.to_string_lossy().into_owned());
            }
        }

        let source_dir_refs: Vec<&str> = source_dir_paths.iter().map(|s| s.as_str()).collect();
        let settings_path = write_settings(&tempdir, &source_dir_refs);
        let bookmap_path = write_bookmap(&tempdir, "");

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        assert_eq!(res.len(), 6);

        for key in &all_epub_paths {
            assert!(res.contains_key(key));
            assert!(matches!(res[key], BookFileTypes::EpubFileType));
        }

        Ok(())
    }

    #[test]
    fn non_empty_bookmap_with_empty_sources() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = TempDir::new()?;

        let settings_path = write_settings(&tempdir, &[]);
        let bookmap_path = write_bookmap(
            &tempdir,
            r#"[
  {
    "uuid": "8c5262ce-c2c2-4601-8c28-ce5e7fc7efec",
    "filename": "03_The_Titan_39_s_Curse",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/8c5262ce-c2c2-4601-8c28-ce5e7fc7efec",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub"
  },
  {
    "uuid": "c22a5f11-1878-4ebd-af5f-c36cad0fedb7",
    "filename": "02_The_Sea_of_Monsters",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/c22a5f11-1878-4ebd-af5f-c36cad0fedb7",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub"
  },
  {
    "uuid": "82c74ef4-cc1b-491a-b0ef-a22a35babb0a",
    "filename": "01_The_Lightning_Thief",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/82c74ef4-cc1b-491a-b0ef-a22a35babb0a",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub"
  },
  {
    "uuid": "3db753a9-43b8-42f1-b4af-9c8a9884e2c0",
    "filename": "04_The_Battle_of_the_Labyrinth",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/3db753a9-43b8-42f1-b4af-9c8a9884e2c0",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub"
  },
  {
    "uuid": "378e305e-9fb4-4efe-9e24-aeb064af8ca9",
    "filename": "05_The_Last_Olympian",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/378e305e-9fb4-4efe-9e24-aeb064af8ca9",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub"
  }
]"#,
        );

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        assert_eq!(res.len(), 5);

        let expected_filepaths = [
            "/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub",
            "/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub",
            "/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub",
            "/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub",
            "/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub",
        ];

        for filepath in expected_filepaths {
            assert!(matches!(res[filepath], BookFileTypes::EpubFileType));
        }

        Ok(())
    }

    #[test]
    fn non_empty_bookmap_with_multiple_sources_and_books() -> Result<(), Box<dyn std::error::Error>>
    {
        let tempdir = TempDir::new()?;

        let bookmap_path = write_bookmap(
            &tempdir,
            r#"[
  {
    "uuid": "8c5262ce-c2c2-4601-8c28-ce5e7fc7efec",
    "filename": "03_The_Titan_39_s_Curse",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/8c5262ce-c2c2-4601-8c28-ce5e7fc7efec",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub"
  },
  {
    "uuid": "c22a5f11-1878-4ebd-af5f-c36cad0fedb7",
    "filename": "02_The_Sea_of_Monsters",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/c22a5f11-1878-4ebd-af5f-c36cad0fedb7",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub"
  },
  {
    "uuid": "82c74ef4-cc1b-491a-b0ef-a22a35babb0a",
    "filename": "01_The_Lightning_Thief",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/82c74ef4-cc1b-491a-b0ef-a22a35babb0a",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub"
  },
  {
    "uuid": "3db753a9-43b8-42f1-b4af-9c8a9884e2c0",
    "filename": "04_The_Battle_of_the_Labyrinth",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/3db753a9-43b8-42f1-b4af-9c8a9884e2c0",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub"
  },
  {
    "uuid": "378e305e-9fb4-4efe-9e24-aeb064af8ca9",
    "filename": "05_The_Last_Olympian",
    "folderpath": "/Users/abhinavkumarsingh/Library/Application Support/com.Blank.Calico/CER_Books/CER_Epubs/378e305e-9fb4-4efe-9e24-aeb064af8ca9",
    "filetype": "epub",
    "filepath": "/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub"
  }
]"#,
        );

        let source_dir_names = ["book_source_1", "book_source_2"];
        let book_names = ["new_book_1.epub", "new_book_2.epub", "new_book_3.epub"];

        let mut all_epub_paths: Vec<String> = Vec::new();
        let mut source_dir_paths: Vec<String> = Vec::new();

        for dir_name in source_dir_names {
            let source_dir = tempdir.path().join(dir_name);
            fs::create_dir(&source_dir)?;
            source_dir_paths.push(source_dir.to_string_lossy().into_owned());
            for book_name in book_names {
                let epub_path = source_dir.join(book_name);
                fs::write(&epub_path, b"")?;
                all_epub_paths.push(epub_path.to_string_lossy().into_owned());
            }
        }

        let source_dir_refs: Vec<&str> = source_dir_paths.iter().map(|s| s.as_str()).collect();
        let settings_path = write_settings(&tempdir, &source_dir_refs);

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        assert_eq!(res.len(), 11);

        for key in &all_epub_paths {
            assert!(res.contains_key(key));
            assert!(matches!(res[key], BookFileTypes::EpubFileType));
        }

        let bookmap_filepaths = [
            "/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub",
            "/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub",
            "/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub",
            "/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub",
            "/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub",
        ];

        for filepath in bookmap_filepaths {
            assert!(matches!(res[filepath], BookFileTypes::EpubFileType));
        }

        Ok(())
    }
}
