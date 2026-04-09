#[derive(Debug)]
pub(crate) enum BookFileTypes {
    EpubFileType,
    UnknownFileType,
}

#[derive(Debug)]
pub(crate) struct BookMetadata {
    title: String,
    author: Option<String>,
    description: Option<String>,
    series: Option<String>,
    series_order_number: Option<usize>,
    subjects: Option<Vec<String>>,
    isbn: Option<String>,
    publisher: Option<String>,
    rights: Option<String>,
}

#[derive(Debug)]
pub(crate) struct BookSection {
    id: String,
    name: Option<String>,
    content: String,
}

#[derive(Debug)]
pub(crate) struct Book {
    metadata: BookMetadata,
    file_type: BookFileTypes,
    content: Vec<BookSection>,
}

impl BookMetadata {
    pub(crate) fn new(
        title: String,
        author: Option<String>,
        description: Option<String>,
        series: Option<String>,
        series_order_number: Option<usize>,
        subjects: Option<Vec<String>>,
        isbn: Option<String>,
        publisher: Option<String>,
        rights: Option<String>,
    ) -> Self {
        Self {
            title: String::from(title),
            author,
            description,
            series,
            series_order_number,
            subjects,
            isbn,
            publisher,
            rights,
        }
    }
}

impl BookSection {
    pub(crate) fn new(id: String, name: Option<String>, content: String) -> Self {
        Self { id, name, content }
    }
}

impl BookFileTypes {
    pub(crate) fn new(file_type: &str) -> Self {
        match file_type {
            "epub" => BookFileTypes::EpubFileType,
            _ => BookFileTypes::UnknownFileType,
        }
    }
}

impl Book {
    pub(crate) fn new(
        metadata: BookMetadata,
        file_type: BookFileTypes,
        content: Vec<BookSection>,
    ) -> Self {
        Self {
            metadata,
            file_type,
            content,
        }
    }
}
