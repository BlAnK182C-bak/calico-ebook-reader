use indexmap::IndexMap;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use xml::reader::{EventReader, XmlEvent};
use zip::ZipArchive;

use super::utils::{extract_attr_value_from_attrs, extract_full_path, extract_metadata_value};
use super::utils::{validate_content_obf, validate_meta_inf, validate_mimetype};
use crate::common::constants::EPUB_ENTRY_POINT;
use crate::common::models::book::{BookMetadata, BookSection};
use crate::common::models::filetypes::BookFileTypes;
use crate::parsers::utils::{get_book_folder_name, get_file_name_from_path};

// structs
#[derive(Debug)]
pub(crate) struct RawEpub {
    file_path: String,
    extracted_directory_path: Option<String>, // The folder in device where the epub is extracted to
    is_validated: bool,
    entry_file_path: Option<String>, // META-INF/container.xml
    rootfile_path: Option<String>,   //content.obf
    spine_to_mainfest_map: IndexMap<String, String>, // using an IndexMap because insertion order
}

// implementations
impl RawEpub {
    // getters
    pub(super) fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub(super) fn get_extracted_directory_path(&self) -> Option<&str> {
        match &self.extracted_directory_path {
            Some(value) => Some(value.as_str()),
            None => None,
        }
    }

    pub(super) fn get_is_validated(&self) -> bool {
        self.is_validated
    }

    pub(super) fn get_rootfile_path(&self) -> Option<&str> {
        match &self.rootfile_path {
            Some(value) => Some(value.as_str()),
            None => None,
        }
    }

    pub(super) fn get_spine_to_manifest_map(&self) -> &IndexMap<String, String> {
        &self.spine_to_mainfest_map
    }
    //setters
    pub(super) fn set_extracted_directory_path(&mut self, path: &str) {
        self.extracted_directory_path = Some(String::from(path));
    }

    pub(super) fn set_is_validated(&mut self, validated_flag: bool) {
        self.is_validated = validated_flag;
    }

    pub(super) fn set_entry_file_path(&mut self, entry_file_path: &str) {
        self.entry_file_path = Some(String::from(entry_file_path));
    }

    pub(super) fn set_rootfile_path(&mut self, rootfile_path: Option<String>) {
        self.rootfile_path = rootfile_path;
    }

    // actual shit
    pub(crate) fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
            extracted_directory_path: None,
            is_validated: false,
            entry_file_path: None,
            rootfile_path: None,
            spine_to_mainfest_map: IndexMap::new(),
        }
    }

    pub(super) fn push_to_spine_manifest_map(&mut self, key: &str, value: &str) {
        self.spine_to_mainfest_map
            .insert(String::from(key), String::from(value));
    }

    // NOTE: using std::io::Error instead of just Error and importing at top level here because XmlEvent throws it's own Error object.
    // This is simply to avoid confusion and so that I have some peace of mind while writing this code.
    pub(super) fn validate(&mut self) -> Result<(), std::io::Error> {
        let edp = match self.get_extracted_directory_path() {
            Some(edp) => PathBuf::from(edp),
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "validate: This epub file doesn't exist",
                ));
            }
        };

        let validation_result = thread::scope(|scope| -> Result<bool, std::io::Error> {
            let mtv_thread_scope = scope.spawn(|| -> Result<bool, std::io::Error> {
                let res = validate_mimetype(edp.to_string_lossy().as_ref())?;
                Ok(res)
            });

            let mi_thread_scope = scope.spawn(|| -> Result<bool, std::io::Error> {
                let res = validate_meta_inf(edp.to_string_lossy().as_ref())?;
                Ok(res)
            });

            let cobf_thread_scope = scope.spawn(|| -> Result<bool, std::io::Error> {
                let res = validate_content_obf(edp.to_string_lossy().as_ref())?;
                Ok(res)
            });

            let is_mimetype_valid: bool = mtv_thread_scope
                .join()
                .map_err(|_| std::io::Error::other("Mimetype validation ran into an error"))??;

            let is_meta_inf_valid: bool = mi_thread_scope
                .join()
                .map_err(|_| std::io::Error::other("META-INF validation ran into an error"))??;

            let is_content_obf_valid: bool = cobf_thread_scope
                .join()
                .map_err(|_| std::io::Error::other("content.obf validation ran into an error"))??;

            Ok(is_meta_inf_valid && is_mimetype_valid && is_content_obf_valid)
        });

        self.set_is_validated(validation_result?);
        Ok(())
    }

    pub(super) fn init(&mut self) -> Result<(), std::io::Error> {
        if !self.is_validated {
            Err(std::io::Error::other(
                "init: The following epub is not validated yet.",
            ))
        } else {
            let edp = match self.get_extracted_directory_path() {
                Some(edp) => PathBuf::from(edp),
                None => {
                    return Err(std::io::Error::new(
                        ErrorKind::NotFound,
                        "init: This epub file doesn't exist",
                    ));
                }
            };

            self.set_entry_file_path(
                Path::new(&edp)
                    .join(EPUB_ENTRY_POINT)
                    .to_string_lossy()
                    .as_ref(),
            );
            self.set_rootfile_path(
                extract_full_path(EventReader::new(File::open(
                    Path::new(&edp).join(EPUB_ENTRY_POINT),
                )?))
                .map(|p| Path::new(&edp).join(p).to_string_lossy().into_owned()),
            );
            Ok(())
        }
    }

    pub(super) fn extract_epub_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let epub_file = fs::File::open(self.get_file_path())?;
        let file_name = get_file_name_from_path(self.get_file_path())?;
        let curr_book_path = get_book_folder_name(BookFileTypes::EpubFileType, file_name)?;

        match fs::exists(&curr_book_path) {
            Ok(file_exists) => {
                if file_exists {
                    println!(
                        "warning: extract_epub_file: This book already exists. Not extracting another folder."
                    );
                } else {
                    fs::create_dir(&curr_book_path)?;
                }
                self.set_extracted_directory_path(curr_book_path.to_string_lossy().as_ref());
            }
            Err(err) => {
                // TODO: Make this error more verbose and specific
                return Err(err.into());
            }
        }

        let mut archive = ZipArchive::new(epub_file)?;
        archive.extract(curr_book_path)?;
        Ok(())
    }

    pub(super) fn extract_epub_metadata(&self) -> Result<BookMetadata, std::io::Error> {
        if self.get_is_validated() {
            let rf = match self.get_rootfile_path() {
                Some(rootfile_path) => rootfile_path,
                None => {
                    return Err(std::io::Error::new(
                        ErrorKind::NotFound,
                        "extract_epub_metadata: Rootfile doesn't exist",
                    ));
                }
            };

            // TODO: Multithreading may not be the right answer here ol' chum. A simple for loop
            // might help.
            thread::scope(|scope| {
                let author_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "creator",
                        Some("role"),
                        Some("aut"),
                    )
                });

                let title_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "title",
                        None,
                        None,
                    )
                });

                let desc_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "description",
                        None,
                        None,
                    )
                });

                // TODO: Not all epubs have a series tag - most of them infact have a meta tag with
                // series as a name
                let series_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "series",
                        None,
                        None,
                    )
                });

                // TODO: Not all epubs have a series_index tag - most of them infact have a meta tag with
                // series_index as a name
                let series_index_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "series_index",
                        None,
                        None,
                    )
                });

                let subject_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "subject",
                        None,
                        None,
                    )
                });

                let isbn_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "identifier",
                        Some("scheme"),
                        Some("ISBN"),
                    )
                });

                let pub_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "publisher",
                        None,
                        None,
                    )
                });

                let rights_handle = scope.spawn(|| {
                    extract_metadata_value(
                        EventReader::new(File::open(&rf).unwrap()),
                        "rights",
                        None,
                        None,
                    )
                });

                Ok(BookMetadata::new(
                    title_handle
                        .join()
                        .unwrap()
                        .unwrap_or_else(|| "Unknown Title".to_string()),
                    author_handle.join().unwrap(),
                    desc_handle.join().unwrap(),
                    series_handle.join().unwrap(),
                    series_index_handle
                        .join()
                        .unwrap()
                        .and_then(|series_order| series_order.parse::<usize>().ok()),
                    subject_handle.join().unwrap().map(|full_subject| {
                        full_subject
                            .split(",")
                            .map(|subject| subject.trim().to_string())
                            .collect()
                    }),
                    isbn_handle.join().unwrap(),
                    pub_handle.join().unwrap(),
                    rights_handle.join().unwrap(),
                ))
            })
        } else {
            Err(std::io::Error::other(
                "extract_epub_metadata: This book is not validated",
            ))
        }
    }
    pub(super) fn map_spine_to_manifest(&mut self) -> Result<(), std::io::Error> {
        let rf = match self.get_rootfile_path() {
            Some(rootfile_path) => rootfile_path,
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "map_spine_to_manifest: Rootfile path not found.",
                ));
            }
        };

        let mut spine_ids: Vec<String> = Vec::new();
        let mut manifest_items: Vec<(String, String)> = Vec::new(); // (id, href)
        let mut content_obf_parser = EventReader::new(File::open(&rf)?);

        let mut is_inside_spine = false;
        let mut is_inside_manifest = false;

        loop {
            match content_obf_parser.next() {
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "spine" => {
                    is_inside_spine = true;
                }
                Ok(XmlEvent::EndElement { ref name }) if name.local_name == "spine" => {
                    is_inside_spine = false;
                }
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "manifest" => {
                    is_inside_manifest = true;
                }
                Ok(XmlEvent::EndElement { ref name }) if name.local_name == "manifest" => {
                    is_inside_manifest = false;
                }
                Ok(XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                }) if is_inside_spine && name.local_name == "itemref" => {
                    if let Ok(idref) = extract_attr_value_from_attrs(attributes, "idref") {
                        spine_ids.push(idref);
                    }
                }
                Ok(XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                }) if is_inside_manifest && name.local_name == "item" => {
                    if let (Ok(id), Ok(href)) = (
                        extract_attr_value_from_attrs(attributes, "id"),
                        extract_attr_value_from_attrs(attributes, "href"),
                    ) {
                        manifest_items.push((id, href));
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                _ => {}
            }
        }

        let manifest_map: std::collections::HashMap<String, String> =
            manifest_items.into_iter().collect();
        for spine_id in &spine_ids {
            if let Some(href) = manifest_map.get(spine_id) {
                self.push_to_spine_manifest_map(spine_id.as_str(), href.as_str());
            }
        }

        Ok(())
    }

    pub(super) fn extract_epub_content(&mut self) -> Result<Vec<BookSection>, std::io::Error> {
        if self.get_is_validated() {
            let mut all_book_sections: Vec<BookSection> = Vec::new();
            self.map_spine_to_manifest()?;
            for (spine_id, path_to_file) in self.get_spine_to_manifest_map() {
                let mut path = match self.get_rootfile_path() {
                    Some(rootfile_path) => PathBuf::from(rootfile_path)
                        .parent()
                        .map(|p| p.to_path_buf())
                        .ok_or_else(|| {
                            std::io::Error::other(
                                "extract_epub_content: Could not get parent directory of rootfile",
                            )
                        })?,
                    None => {
                        return Err(std::io::Error::other(
                            "extract_epub_content: No extracted directory path.",
                        ));
                    }
                };

                path.push(path_to_file);
                let mut section_parser = EventReader::new(File::open(&path).map_err(|err| {
                    std::io::Error::other(format!("Failed to open file: {:?}: {}", path, err))
                })?);
                let mut section_content = String::new();
                let mut is_inside_body = false;
                loop {
                    match section_parser.next() {
                        Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "body" => {
                            is_inside_body = true;
                        }

                        Ok(XmlEvent::EndElement { name }) if name.local_name == "body" => {
                            break;
                        }

                        Ok(XmlEvent::Characters(text)) if is_inside_body => {
                            section_content.push_str(&text);
                        }

                        Ok(XmlEvent::EndElement { .. }) if is_inside_body => {
                            section_content.push_str("\n");
                        }

                        Ok(XmlEvent::EndDocument) => {
                            break;
                        }
                        _ => {}
                    }
                }
                let section = BookSection::new(String::from(spine_id), None, section_content);
                all_book_sections.push(section);
            }
            Ok(all_book_sections)
        } else {
            return Err(std::io::Error::other(
                "extract_epub_content: This epub is not validated.",
            ));
        }
    }
}
