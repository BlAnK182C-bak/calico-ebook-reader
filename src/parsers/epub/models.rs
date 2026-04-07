use std::fs::{File, exists};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::thread;
use xml::reader::EventReader;

use crate::misc::constants::{EPUB_ENTRY_POINT, EPUB_MIMETYPE};
use crate::parsers::epub::extractors::extract_full_path;

// structs
#[derive(Debug)]
pub(crate) struct RawEpub {
    file_path: String,
    extracted_directory_path: Option<String>,
    is_validated: bool,
    entry_file_path: Option<String>, // META-INF/container.xml
    rootfile_path: Option<String>,   //content.obf
}

// helpers
// god help our code readability
fn validate_mimetype(path: &str) -> Result<bool, std::io::Error> {
    let mut mimetype_file = File::open(Path::new(path).join("mimetype"))?;
    let mut mimetype_contents = String::new();
    mimetype_file.read_to_string(&mut mimetype_contents)?;
    Ok(mimetype_contents == EPUB_MIMETYPE)
}

fn validate_meta_inf(path: &str) -> Result<bool, std::io::Error> {
    let does_entry_point_exist = exists(Path::new(path).join(EPUB_ENTRY_POINT))?;
    Ok(does_entry_point_exist)
}

fn validate_content_obf(path: &str) -> Result<bool, std::io::Error> {
    let container_xml_parser =
        EventReader::new(File::open(Path::new(path).join(EPUB_ENTRY_POINT))?);

    let full_path = extract_full_path(container_xml_parser);

    match full_path {
        Some(_) => Ok(true),
        None => Ok(false),
    }
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

    pub(super) fn get_entry_point(&self) -> Option<&str> {
        match &self.entry_file_path {
            Some(entry) => Some(entry.as_str()),
            None => None,
        }
    }

    pub(super) fn get_rootfile_path(&self) -> Option<&str> {
        match &self.rootfile_path {
            Some(value) => Some(value.as_str()),
            None => None,
        }
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
    pub(super) fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
            extracted_directory_path: None,
            is_validated: false,
            entry_file_path: None,
            rootfile_path: None,
        }
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
}
