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
    let container_xml_parser =
        EventReader::new(File::open(Path::new(path).join(EPUB_ENTRY_POINT))?);

    let full_path = extract_full_path(container_xml_parser);

    match full_path {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
