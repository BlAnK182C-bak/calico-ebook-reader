use indexmap::IndexMap;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
use zip::ZipArchive;

use super::models::RawEpub;
use super::utils::{
    extract_attr_value_from_attrs, extract_full_path, validate_content_obf, validate_meta_inf,
    validate_mimetype,
};
use crate::common::constants::EPUB_ENTRY_POINT;
use crate::common::models::book::Book;
use crate::common::models::book::{BookMetadata, BookSection};
use crate::common::models::filetypes::BookFileTypes;
use crate::parsers::models::ParserEngine;
use crate::parsers::utils::get_book_folder_path;

// TODO: When I load books, all of them get loaded into memory - instead cache loaded books in a
// binary maybe and call said binary when someone selects a book

impl ParserEngine for RawEpub {
    fn parse(&mut self) -> Result<Book, Box<dyn std::error::Error>> {
        self.extract_epub_file()?;
        self.validate()?;
        self.init()?;
        let new_epub_metadata = self.extract_epub_metadata()?;
        let new_epub_sections = self.extract_epub_content()?;
        let new_epub_file_type = "epub";

        let book = Book::new(
            new_epub_metadata,
            BookFileTypes::new(new_epub_file_type),
            new_epub_sections,
        );

        Ok(book)
    }
}

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

    pub(super) fn get_rootfile_path(&self) -> Result<&str, std::io::Error> {
        match &self.rootfile_path {
            Some(value) => Ok(value.as_str()),
            None => Err(std::io::Error::other(
                "get_rootfile_path: Rootfile path has not been set",
            )),
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

        let is_mimetype_valid = validate_mimetype(&edp.to_string_lossy())?;
        let is_meta_inf_valid = validate_meta_inf(&edp.to_string_lossy())?;
        let is_content_obf_valid = validate_content_obf(&edp.to_string_lossy())?;

        self.set_is_validated(is_meta_inf_valid && is_mimetype_valid && is_content_obf_valid);
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
                extract_full_path(&mut EventReader::new(File::open(
                    Path::new(&edp).join(EPUB_ENTRY_POINT),
                )?))
                .map(|p| Path::new(&edp).join(p).to_string_lossy().into_owned()),
            );
            Ok(())
        }
    }

    pub(super) fn extract_epub_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let epub_file = fs::File::open(self.get_file_path())?;
        let curr_book_path =
            get_book_folder_path(BookFileTypes::EpubFileType, self.get_file_path())?;

        let file_exists = fs::exists(&curr_book_path)?;
        if file_exists {
            println!(
                "warning: extract_epub_file: This book already exists. Not extracting another folder."
            );
        } else {
            fs::create_dir(&curr_book_path)?;
            let mut archive = ZipArchive::new(epub_file)?;
            archive.extract(&curr_book_path)?;
        }
        self.set_extracted_directory_path(curr_book_path.to_string_lossy().as_ref());
        Ok(())
    }

    pub(super) fn extract_epub_metadata(&self) -> Result<BookMetadata, std::io::Error> {
        if self.get_is_validated() {
            let rf = self.get_rootfile_path()?;

            let mut event_reader = EventReader::new(File::open(rf).unwrap());

            let mut title: String = String::from("Unknown Title");
            let mut author: Option<String> = Some(String::from("Unknown author"));
            let mut description: Option<String> = Some(String::from("N/A"));
            let mut series: Option<String> = Some(String::from("N/A"));
            let mut series_order_number: Option<usize> = Some(0usize);
            let mut subjects: Option<Vec<String>> = Some(Vec::new());
            let mut isbn: Option<String> = Some(String::from("N/A"));
            let mut publisher: Option<String> = Some(String::from("N/A"));
            let mut rights: Option<String> = Some(String::from("N/A"));

            let mut is_inside_metadata = false;

            loop {
                match event_reader.next() {
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "metadata" =>
                    {
                        is_inside_metadata = true;
                    }

                    Ok(XmlEvent::StartElement { ref name, .. })
                        if is_inside_metadata && name.local_name == "title" =>
                    {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            title = text;
                        }
                    }

                    Ok(XmlEvent::StartElement { ref name, .. })
                        if is_inside_metadata && name.local_name == "description" =>
                    {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            description = Some(text);
                        }
                    }

                    Ok(XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    }) if is_inside_metadata && name.local_name == "creator" => {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            author = Some(text);
                        }
                    }

                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "subject" => {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            subjects.get_or_insert_with(Vec::new).push(text);
                        }
                    }

                    Ok(XmlEvent::StartElement { ref name, .. })
                        if is_inside_metadata && name.local_name == "publisher" =>
                    {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            publisher = Some(text);
                        }
                    }

                    Ok(XmlEvent::StartElement {
                        ref name,
                        attributes,
                        ..
                    }) if is_inside_metadata && name.local_name == "identifier" => {
                        let has_isbn = attributes.iter().any(|a| a.value == "isbn");
                        if has_isbn && let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            isbn = Some(text);
                        }
                    }

                    Ok(XmlEvent::StartElement { ref name, .. })
                        if is_inside_metadata && name.local_name == "rights" =>
                    {
                        if let Ok(XmlEvent::Characters(text)) = event_reader.next() {
                            rights = Some(text);
                        }
                    }

                    Ok(XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    }) if is_inside_metadata && name.local_name == "meta" => {
                        let meta_name = attributes
                            .iter()
                            .find(|a| a.name.local_name == "name")
                            .map(|a| &a.value);

                        let meta_content = attributes
                            .iter()
                            .find(|a| a.name.local_name == "content")
                            .map(|a| &a.value);

                        if let (Some(n), Some(c)) = (meta_name, meta_content) {
                            match n.as_str() {
                                "calibre:series" => series = Some(c.into()),
                                "calibre:series_index" => {
                                    series_order_number = Some(c.parse::<f32>().unwrap() as usize)
                                }
                                _ => {}
                            }
                        }
                    }

                    Ok(XmlEvent::EndDocument) => {
                        break;
                    }
                    _ => {}
                }
            }

            Ok(BookMetadata::new(
                title,
                author,
                description,
                series,
                series_order_number,
                subjects,
                isbn,
                publisher,
                rights,
            ))
        } else {
            Err(std::io::Error::other(
                "extract_epub_metadata: This book is not validated",
            ))
        }
    }
    pub(super) fn map_spine_to_manifest(&mut self) -> Result<(), std::io::Error> {
        let rf = self.get_rootfile_path()?;

        let mut spine_ids: Vec<String> = Vec::new();
        let mut manifest_items: Vec<(String, String)> = Vec::new(); // (id, href)
        let mut content_obf_parser = EventReader::new(File::open(rf)?);

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
            let manifest = self.get_spine_to_manifest_map();

            let base_dir_path = PathBuf::from(self.get_rootfile_path()?)
                .parent()
                .map(|p| p.to_path_buf()).ok_or_else(|| std::io::Error::other("extract_epub_content: rootfile_path could not be mapped to PathBuf correctly"))?;

            // you paid for the whole CPU, and you bet your sweet bippy this might use all of it
            // TODO: How the fuck does one achieve concurrency without risk of excess CPU thread
            // usage :")) - I guess you don't huh
            // Still I don't like that we are using n * k amount of threads possible at a time where
            // n is number of books and k the number of chapters

            let results: Vec<Result<BookSection, std::io::Error>> = manifest
                .par_iter()
                .map(
                    |(spine_id, path_to_file)| -> Result<BookSection, std::io::Error> {
                        let path = base_dir_path.join(path_to_file);
                        let mut section_parser =
                            EventReader::new(File::open(&path).map_err(|err| {
                                std::io::Error::other(format!(
                                    "Failed to open file: {:?}: {}",
                                    path, err
                                ))
                            })?);
                        let mut section_content = String::new();
                        let mut is_inside_body = false;
                        loop {
                            match section_parser.next() {
                                Ok(XmlEvent::StartElement { name, .. })
                                    if name.local_name == "body" =>
                                {
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
                        Ok(BookSection::new(
                            String::from(spine_id),
                            None,
                            section_content,
                        ))
                    },
                )
                .collect();

            for res in results {
                all_book_sections.push(res?);
            }

            Ok(all_book_sections)
        } else {
            return Err(std::io::Error::other(
                "extract_epub_content: This epub is not validated.",
            ));
        }
    }
}
