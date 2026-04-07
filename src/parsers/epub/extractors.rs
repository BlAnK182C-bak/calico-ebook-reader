use std::fs::{self, File};
use std::io::ErrorKind;
use std::thread;
use xml::EventReader;
use xml::reader::XmlEvent;
use zip::ZipArchive;

use super::models::RawEpub;
use crate::misc::models::{BookFileTypes, BookMetadata, BookSections};
use crate::misc::utils::{get_book_folder_name, get_file_name_from_path};

// helpers
pub(super) fn extract_full_path(container_xml_parser: EventReader<File>) -> Option<String> {
    container_xml_parser
        .into_iter()
        .find(|element| {
            matches!(
                element,
                Ok(XmlEvent::StartElement {
                    name,
                    ..
                }) if name.local_name == "rootfile"
            )
        })
        .and_then(|event| event.ok())
        .and_then(|event| {
            if let XmlEvent::StartElement { attributes, .. } = event {
                attributes
                    .into_iter()
                    .find(|attr| attr.name.local_name == "full-path")
                    .map(|attr| attr.value)
            } else {
                None
            }
        })
}

// TODO: content_obf_parser needs to be a &mut EventReader not a EventReader
pub(super) fn extract_metadata_value<'a>(
    content_obf_parser: EventReader<File>,
    tag_name: &'a str,
    attr_name: Option<&'a str>,
    attr_value: Option<&'a str>,
) -> Option<String> {
    let mut inside_metadata_tag = false;
    let mut iter = content_obf_parser.into_iter();

    while let Some(Ok(event)) = iter.next() {
        match event {
            XmlEvent::StartElement { ref name, .. } if name.local_name == "metadata" => {
                inside_metadata_tag = true;
            }
            XmlEvent::EndElement { ref name } if name.local_name == "metadata" => {
                break;
            }

            XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            } if inside_metadata_tag && name.local_name == tag_name => {
                let matches = match (attr_name, attr_value) {
                    (Some(a_name), Some(a_val)) => attributes
                        .iter()
                        .any(|attr| attr.name.local_name == a_name && attr.value == a_val),
                    _ => true,
                };

                if matches && let Some(Ok(XmlEvent::Characters(text))) = iter.next() {
                    return Some(text);
                }
            }

            _ => {}
        }
    }
    None
}

// implementations
impl RawEpub {
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

    pub(super) fn extract_epub_content(&self) -> BookSections {
        todo!();
    }
}
