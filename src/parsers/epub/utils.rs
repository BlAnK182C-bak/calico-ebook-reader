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
    loop {
        match container_xml_parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "rootfile" {
                    return attributes
                        .into_iter()
                        .find(|attr| attr.name.local_name == "full-path")
                        .map(|attr| attr.value);
                }
            }

            Ok(XmlEvent::EndDocument) => {
                return None;
            }

            Err(_) => {
                return None;
            }

            _ => {}
        }
    }
}
// god help our code readability
pub(super) fn validate_mimetype(extracted_dir_path: &str) -> Result<bool, std::io::Error> {
    let mut mimetype_file = File::open(Path::new(extracted_dir_path).join("mimetype"))?;
    let mut mimetype_contents = String::new();
    mimetype_file.read_to_string(&mut mimetype_contents)?;
    Ok(mimetype_contents == EPUB_MIMETYPE)
}

pub(super) fn validate_meta_inf(extracted_dir_path: &str) -> Result<bool, std::io::Error> {
    let does_entry_point_exist = exists(Path::new(extracted_dir_path).join(EPUB_ENTRY_POINT))?;
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

#[cfg(test)]
mod extract_attr_value_from_attrs_tests {
    // AI generated tests
    use xml::attribute::OwnedAttribute;
    use xml::name::OwnedName;

    use super::extract_attr_value_from_attrs;

    #[test]
    fn extracts_attribute_value_successfully() {
        let attributes = vec![
            OwnedAttribute {
                name: OwnedName::local("id"),
                value: "123".to_string(),
            },
            OwnedAttribute {
                name: OwnedName::local("full-path"),
                value: "OPS/content.opf".to_string(),
            },
        ];

        let result = extract_attr_value_from_attrs(&attributes, "full-path").unwrap();

        assert_eq!(result, "OPS/content.opf");
    }

    #[test]
    fn returns_error_when_attribute_missing() {
        let attributes = vec![OwnedAttribute {
            name: OwnedName::local("id"),
            value: "123".to_string(),
        }];

        let result = extract_attr_value_from_attrs(&attributes, "full-path");

        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();

        assert_eq!(
            error_message,
            "extract_attr_value_from_attrs: Not found attribute: full-path"
        );
    }

    #[test]
    fn returns_first_matching_attribute() {
        let attributes = vec![
            OwnedAttribute {
                name: OwnedName::local("full-path"),
                value: "first.opf".to_string(),
            },
            OwnedAttribute {
                name: OwnedName::local("full-path"),
                value: "second.opf".to_string(),
            },
        ];

        let result = extract_attr_value_from_attrs(&attributes, "full-path").unwrap();

        assert_eq!(result, "first.opf");
    }

    #[test]
    fn works_with_empty_attribute_list() {
        let attributes = vec![];

        let result = extract_attr_value_from_attrs(&attributes, "full-path");

        assert!(result.is_err());
    }

    #[test]
    fn attribute_name_matching_is_case_sensitive() {
        let attributes = vec![OwnedAttribute {
            name: OwnedName::local("Full-Path"),
            value: "OPS/content.opf".to_string(),
        }];

        let result = extract_attr_value_from_attrs(&attributes, "full-path");

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod extract_full_path_tests {
    // AI generated tests

    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use xml::EventReader;

    use super::extract_full_path;

    #[test]
    fn extracts_full_path_successfully() {
        let mut temp_file = NamedTempFile::new().unwrap();

        write!(
            temp_file,
            r#"
            <container>
                <rootfiles>
                    <rootfile full-path="OPS/content.opf" />
                </rootfiles>
            </container>
            "#
        )
        .unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let mut parser = EventReader::new(file);

        let result = extract_full_path(&mut parser);

        assert_eq!(result, Some("OPS/content.opf".to_string()));
    }

    #[test]
    fn returns_none_when_rootfile_missing() {
        let mut temp_file = NamedTempFile::new().unwrap();

        write!(
            temp_file,
            r#"
            <container>
                <rootfiles>
                </rootfiles>
            </container>
            "#
        )
        .unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let mut parser = EventReader::new(file);

        let result = extract_full_path(&mut parser);

        assert_eq!(result, None);
    }

    #[test]
    fn returns_none_when_full_path_missing() {
        let mut temp_file = NamedTempFile::new().unwrap();

        write!(
            temp_file,
            r#"
            <container>
                <rootfiles>
                    <rootfile />
                </rootfiles>
            </container>
            "#
        )
        .unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let mut parser = EventReader::new(file);

        let result = extract_full_path(&mut parser);

        assert_eq!(result, None);
    }

    #[test]
    fn returns_first_rootfile_when_multiple_exist() {
        let mut temp_file = NamedTempFile::new().unwrap();

        write!(
            temp_file,
            r#"
            <container>
                <rootfiles>
                    <rootfile full-path="first.opf" />
                    <rootfile full-path="second.opf" />
                </rootfiles>
            </container>
            "#
        )
        .unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let mut parser = EventReader::new(file);

        let result = extract_full_path(&mut parser);

        assert_eq!(result, Some("first.opf".to_string()));
    }
}

#[cfg(test)]
mod validate_mimetype_tests {
    use std::fs;
    use tempfile::tempdir;

    use crate::common::constants::EPUB_MIMETYPE;

    #[test]
    fn not_epub_mimetype() -> Result<(), std::io::Error> {
        let tempdir = tempdir()?;
        let file_path = tempdir.path().join("mimetype");
        fs::write(&file_path, "someotherfiletype")?;

        let res = super::validate_mimetype(&tempdir.path().to_string_lossy().to_string())?;
        assert_eq!(res, false);

        Ok(())
    }

    #[test]
    fn is_epub_mimetype() -> Result<(), std::io::Error> {
        let tempdir = tempdir()?;
        let file_path = tempdir.path().join("mimetype");
        fs::write(&file_path, EPUB_MIMETYPE)?;

        let res = super::validate_mimetype(&tempdir.path().to_string_lossy().to_string())?;
        assert_eq!(res, true);

        Ok(())
    }
}
