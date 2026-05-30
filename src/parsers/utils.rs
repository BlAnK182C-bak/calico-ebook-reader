use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

use crate::common::models::filetypes::BookFileTypes;
use crate::common::models::settings::BookMap;

pub(crate) fn get_book_folder_path(
    file_type: &BookFileTypes,
    filepath: &str,
    bookmap_file_path: &PathBuf,
    epub_dir_path: &PathBuf,
) -> Result<PathBuf, std::io::Error> {
    let file_name_without_extension =
        get_file_name_from_path(filepath)?
            .split(".")
            .next()
            .ok_or(std::io::Error::other(
                "get_book_folder_name: Something went wrong while getting filename",
            ))?;

    if file_name_without_extension.trim().is_empty() {
        Err(std::io::Error::other(
            "get_book_folder_name: This file has an emtpy name",
        ))
    } else {
        match file_type {
            BookFileTypes::EpubFileType => Ok(epub_dir_path.join(Path::new(
                &get_book_uuid(
                    "epub",
                    file_name_without_extension,
                    &epub_dir_path,
                    filepath,
                    &bookmap_file_path,
                )?
                .to_string(),
            ))),
            BookFileTypes::UnknownFileType => Err(std::io::Error::other(
                "get_book_folder_name: Unknown file types",
            )),
        }
    }
}

static BOOKMAP_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub(crate) fn get_book_uuid(
    filetype: &str,
    file_name: &str,
    parent_file_path: &PathBuf,
    filepath: &str,
    bookmap_file_path: &PathBuf,
) -> Result<Uuid, std::io::Error> {
    let lock = BOOKMAP_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();

    let bookmap_exists = fs::exists(&bookmap_file_path)?;
    if bookmap_exists {
        let file_contents = fs::read_to_string(&bookmap_file_path)?;
        let mut book_map: Vec<BookMap> = serde_json::from_str(&file_contents).unwrap_or(vec![]);

        // TODO: lookup can change from O(n) to O(1) if we use hashmaps - whether we do that or not,
        // we can optimize existing lookup much better
        for bookmap_field in book_map.iter() {
            if bookmap_field.get_filename() == file_name {
                return Ok(bookmap_field.get_uuid());
            }
        }

        let new_book_uuid = Uuid::new_v4();

        let file_path = parent_file_path.join(new_book_uuid.to_string());

        book_map.push(BookMap::new(
            new_book_uuid,
            file_name,
            file_path.to_str().unwrap(),
            filetype,
            filepath,
        ));

        let tmp = format!("{}.tmp", bookmap_file_path.to_string_lossy());
        let file = fs::File::create(&tmp)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &book_map).map_err(|e| std::io::Error::other(e))?;
        fs::rename(&tmp, bookmap_file_path.as_path())?;

        Ok(new_book_uuid)
    } else {
        Err(std::io::Error::other(
            "get_book_uuid: bookmap file doesn't exist",
        ))
    }
}

pub(crate) fn get_file_name_from_path(file_path: &str) -> Result<&str, std::io::Error> {
    match file_path.split("/").last() {
        Some(file_name) => Ok(file_name),
        None => Err(std::io::Error::other(
            "get_file_name_from_path: Couldn't find a file of this file path",
        )),
    }
}

#[cfg(test)]
mod get_file_name_from_path_tests {
    #[test]
    fn filename_has_no_slash() -> Result<(), std::io::Error> {
        let file_path = "somefile.txt";
        let res = super::get_file_name_from_path(file_path)?;
        assert_eq!(file_path, res);
        Ok(())
    }

    #[test]
    fn multiple_slashes_in_file_path() -> Result<(), std::io::Error> {
        let file_path = "some_dir1/some_dir2/some_dir3/jo_mama/hi_have_you_met_ted/file.txt";
        let file_name = "file.txt";
        let res = super::get_file_name_from_path(file_path)?;
        assert_eq!(res, file_name);
        Ok(())
    }

    #[test]
    fn filename_get_when_empty() -> Result<(), std::io::Error> {
        let file_path = "";
        let res = super::get_file_name_from_path(file_path)?;
        assert_eq!(res, "");
        Ok(())
    }
}

#[cfg(test)]
mod get_book_uuid_tests {
    use crate::common::utils::tests::write_bookmap;
    use std::fs;
    use std::sync::Arc;
    use std::thread;
    use tempfile::TempDir;
    use uuid::Uuid;

    use crate::common::models::settings::BookMap;

    #[test]
    fn a_new_book() -> Result<(), std::io::Error> {
        let tempdir = TempDir::new()?;
        let bookmap_path = write_bookmap(&tempdir, "[]");
        let parent_path = tempdir.path().to_path_buf();
        let filename = "somenewfile.epub";
        let filepath = bookmap_path.join(filename);

        let res = super::get_book_uuid(
            "epub",
            filename,
            &parent_path,
            &filepath.to_string_lossy().to_string(),
            &bookmap_path,
        )?;

        assert_ne!(res, Uuid::nil());
        Ok(())
    }

    #[test]
    fn an_old_book() -> Result<(), std::io::Error> {
        let tempdir = TempDir::new()?;
        let bookmap_path = write_bookmap(
            &tempdir,
            r#"[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "filename": "someoldfile.epub",
    "folderpath": "/some/folder/path",
    "filetype": "epub",
    "filepath": "/some/path/someoldfile.epub"
  }
]"#,
        );
        let parent_path = tempdir.path().to_path_buf();
        let filename = "someoldfile.epub";

        let res = super::get_book_uuid(
            "epub",
            filename,
            &parent_path,
            "/some/path/someoldfile.epub",
            &bookmap_path,
        )?;

        let expected = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn concurrency_testing_with_fifty_books() -> Result<(), std::io::Error> {
        let tempdir = TempDir::new()?;
        let bookmap_path = Arc::new(write_bookmap(&tempdir, "[]"));
        let parent_path = Arc::new(tempdir.path().to_path_buf());

        let handles: Vec<_> = (0..50)
            .map(|i| {
                let bookmap_path = Arc::clone(&bookmap_path);
                let parent_path = Arc::clone(&parent_path);
                thread::spawn(move || {
                    let filename = format!("book_{}", i);
                    let filepath = format!("/some/path/book_{}.epub", i);
                    super::get_book_uuid("epub", &filename, &parent_path, &filepath, &bookmap_path)
                })
            })
            .collect();

        let uuids: Vec<Uuid> = handles
            .into_iter()
            .map(|h| {
                h.join()
                    .expect("Something went wrong - a thread panic perhaps")
                    .expect("get_book_uuid failed")
            })
            .collect();

        assert_eq!(uuids.len(), 50);

        let bookmap_content = fs::read_to_string(bookmap_path.as_ref())?;
        let book_map: Vec<BookMap> = serde_json::from_str(&bookmap_content).unwrap();
        assert_eq!(book_map.len(), 50);
        Ok(())
    }
}

#[cfg(test)]
mod get_book_folder_path_tests {
    use crate::common::models::filetypes::BookFileTypes;
    use crate::common::utils::tests::{make_ebooks_dir, write_bookmap};
    use tempfile::TempDir;

    #[test]
    fn other_book_type_than_known() -> Result<(), std::io::Error> {
        let filename = "somerandomfile.someext";
        let filepath = format!("I/already/befriended/your/mom/lasnight/{}", filename);
        let file_type = BookFileTypes::new(filename.split(".").last().ok_or(
            std::io::Error::other("Something went wrong while splitting"),
        )?);
        let tempdir = TempDir::new()?;
        let bookmap_path = write_bookmap(&tempdir, "[]");
        let epub_dir_path = make_ebooks_dir(&tempdir);

        let err = super::get_book_folder_path(&file_type, &filepath, &bookmap_path, &epub_dir_path)
            .unwrap_err();

        assert!(
            err.to_string().contains("Unknown file types"),
            "Some unknown error occurred: {err}"
        );
        Ok(())
    }

    #[test]
    fn empty_file_name() -> Result<(), std::io::Error> {
        let filenames: [&str; 3] = [".epub", ".", ""];

        for filename in filenames {
            let filepath = format!("I/already/befriended/your/mom/lasnight/{}", filename);
            let file_type = BookFileTypes::new(filename.split(".").last().ok_or(
                std::io::Error::other("Something went wrong while splitting"),
            )?);
            let tempdir = TempDir::new()?;
            let bookmap_path = write_bookmap(&tempdir, "[]");
            let epub_dir_path = make_ebooks_dir(&tempdir);

            let err =
                super::get_book_folder_path(&file_type, &filepath, &bookmap_path, &epub_dir_path)
                    .unwrap_err();

            assert!(
                err.to_string().contains("This file has an emtpy name"),
                "Some unknown error occurred: {err}"
            );
        }

        Ok(())
    }
}
