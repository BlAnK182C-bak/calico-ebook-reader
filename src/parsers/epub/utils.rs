use std::fs::{File, exists};
use std::io::Read;
use std::path::Path;
use xml::reader::XmlEvent;
use xml::{EventReader, attribute};

use crate::common::constants::{EPUB_ENTRY_POINT, EPUB_MIMETYPE};

pub(super) fn extract_attr_value_from_attrs(
    attributes: &Vec<attribute::OwnedAttribute>,
    attr_name: &str,
) -> Result<String, std::io::Error> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == attr_name)
        .map(|attr| attr.value.clone())
        .ok_or_else(|| {
            std::io::Error::other(format!(
                "extract_attr_value_from_attrs: Not found attribute: {}",
                attr_name
            ))
        })
}

pub(super) fn extract_full_path(container_xml_parser: &mut EventReader<File>) -> Option<String> {
    while let Ok(event) = container_xml_parser.next() {
        if let XmlEvent::StartElement {
            name, attributes, ..
        } = event
        {
            if name.local_name == "rootfile" {
                return attributes
                    .into_iter()
                    .find(|attr| attr.name.local_name == "full-path")
                    .map(|attr| attr.value);
            }
        }
    }
    None
}

// god help our code readability
pub(super) fn validate_mimetype(path: &str) -> Result<bool, std::io::Error> {
    let mut mimetype_file = File::open(Path::new(path).join("mimetype"))?;
    let mut mimetype_contents = String::new();
    mimetype_file.read_to_string(&mut mimetype_contents)?;
    Ok(mimetype_contents == EPUB_MIMETYPE)
}

pub(super) fn validate_meta_inf(path: &str) -> Result<bool, std::io::Error> {
    let does_entry_point_exist = exists(Path::new(path).join(EPUB_ENTRY_POINT))?;
    Ok(does_entry_point_exist)
}

pub(super) fn validate_content_obf(path: &str) -> Result<bool, std::io::Error> {
    let mut container_xml_parser =
        EventReader::new(File::open(Path::new(path).join(EPUB_ENTRY_POINT))?);

    let full_path = extract_full_path(&mut container_xml_parser);

    match full_path {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
