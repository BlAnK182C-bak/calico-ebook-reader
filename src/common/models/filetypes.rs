#[derive(Debug)]
pub(crate) enum BookFileTypes {
    EpubFileType,
    UnknownFileType,
}

impl BookFileTypes {
    pub(crate) fn new(file_type: &str) -> Self {
        match file_type {
            "epub" => BookFileTypes::EpubFileType,
            _ => BookFileTypes::UnknownFileType,
        }
    }
}
