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
    use std::{fs, path::PathBuf};
    use tempfile::TempDir;

    use crate::common::models::filetypes::BookFileTypes;

    fn write_settings(tempdir: &TempDir, source_paths: &[&str]) -> PathBuf {
        let path = tempdir.path().join("settings.toml");
        let paths_toml = source_paths
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(", ");
        fs::write(&path, format!("[sources]\nsource_paths = [{}]", paths_toml)).unwrap();
        path
    }

    fn write_bookmap(tempdir: &TempDir, bookmap_content: &str) -> PathBuf {
        let path = tempdir.path().join("bookmap.json");
        fs::write(&path, bookmap_content).unwrap();
        path
    }

    #[test]
    fn existing_sources_no_exsting_bookmap() -> Result<(), Box<dyn std::error::Error>> {
        // Expectation: 3 epubs in a source file - all of them should be returned in a hashmap and
        // with their respective file types

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
        // Expectation: Error on bookmap file path not existing

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
        // Expectation: Error on settings file path not existing

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
        // Expectation: resulting hashmap should have no items
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
        let source_dir_1 = tempdir.path().join("book_source_1");
        fs::create_dir(&source_dir_1)?;
        let epub_path_1 = source_dir_1.join("new_book_1.epub");
        let epub_path_2 = source_dir_1.join("new_book_2.epub");
        let epub_path_3 = source_dir_1.join("new_book_3.epub");
        fs::write(&epub_path_1, b"")?;
        fs::write(&epub_path_2, b"")?;
        fs::write(&epub_path_3, b"")?;

        let source_dir_2 = tempdir.path().join("book_source_2");
        fs::create_dir(&source_dir_2)?;
        let epub_path_4 = source_dir_2.join("new_book_1.epub");
        let epub_path_5 = source_dir_2.join("new_book_2.epub");
        let epub_path_6 = source_dir_2.join("new_book_3.epub");
        fs::write(&epub_path_4, b"")?;
        fs::write(&epub_path_5, b"")?;
        fs::write(&epub_path_6, b"")?;

        let settings_path = tempdir.path().join("settings.toml");
        write_settings(
            &tempdir,
            &[
                source_dir_1.to_str().unwrap(),
                source_dir_2.to_str().unwrap(),
            ],
        );
        let bookmap_path = tempdir.path().join("bookmap.json");
        write_bookmap(&tempdir, "");

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;
        assert_eq!(res.len(), 6);

        let key_1 = epub_path_1.to_string_lossy().to_string();
        let key_2 = epub_path_2.to_string_lossy().to_string();
        let key_3 = epub_path_3.to_string_lossy().to_string();
        let key_4 = epub_path_4.to_string_lossy().to_string();
        let key_5 = epub_path_5.to_string_lossy().to_string();
        let key_6 = epub_path_6.to_string_lossy().to_string();

        assert!(res.contains_key(&key_1));
        assert!(res.contains_key(&key_2));
        assert!(res.contains_key(&key_3));
        assert!(res.contains_key(&key_4));
        assert!(res.contains_key(&key_5));
        assert!(res.contains_key(&key_6));
        assert!(matches!(res[&key_1], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_2], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_3], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_4], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_5], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_6], BookFileTypes::EpubFileType));
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
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub"],
            BookFileTypes::EpubFileType
        ));
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

        let source_dir_1 = tempdir.path().join("book_source_1");
        fs::create_dir(&source_dir_1)?;
        let epub_path_1 = source_dir_1.join("new_book_1.epub");
        let epub_path_2 = source_dir_1.join("new_book_2.epub");
        let epub_path_3 = source_dir_1.join("new_book_3.epub");
        fs::write(&epub_path_1, b"")?;
        fs::write(&epub_path_2, b"")?;
        fs::write(&epub_path_3, b"")?;

        let source_dir_2 = tempdir.path().join("book_source_2");
        fs::create_dir(&source_dir_2)?;
        let epub_path_4 = source_dir_2.join("new_book_1.epub");
        let epub_path_5 = source_dir_2.join("new_book_2.epub");
        let epub_path_6 = source_dir_2.join("new_book_3.epub");
        fs::write(&epub_path_4, b"")?;
        fs::write(&epub_path_5, b"")?;
        fs::write(&epub_path_6, b"")?;

        let settings_path = tempdir.path().join("settings.toml");
        write_settings(
            &tempdir,
            &[
                source_dir_1.to_str().unwrap(),
                source_dir_2.to_str().unwrap(),
            ],
        );

        let res = super::scan_sources_for_books(bookmap_path, settings_path)?;

        let key_1 = epub_path_1.to_string_lossy().to_string();
        let key_2 = epub_path_2.to_string_lossy().to_string();
        let key_3 = epub_path_3.to_string_lossy().to_string();
        let key_4 = epub_path_4.to_string_lossy().to_string();
        let key_5 = epub_path_5.to_string_lossy().to_string();
        let key_6 = epub_path_6.to_string_lossy().to_string();

        assert_eq!(res.len(), 11);
        assert!(res.contains_key(&key_1));
        assert!(res.contains_key(&key_2));
        assert!(res.contains_key(&key_3));
        assert!(res.contains_key(&key_4));
        assert!(res.contains_key(&key_5));
        assert!(res.contains_key(&key_6));
        assert!(matches!(res[&key_1], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_2], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_3], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_4], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_5], BookFileTypes::EpubFileType));
        assert!(matches!(res[&key_6], BookFileTypes::EpubFileType));

        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/03_The_Titan_39_s_Curse.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/02_The_Sea_of_Monsters.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/01_The_Lightning_Thief.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/04_The_Battle_of_the_Labyrinth.epub"],
            BookFileTypes::EpubFileType
        ));
        assert!(matches!(
            res["/Users/abhinavkumarsingh/Documents/05_The_Last_Olympian.epub"],
            BookFileTypes::EpubFileType
        ));

        Ok(())
    }
}
