pub(crate) trait ParserEngine {
    fn parse(&mut self) -> Result<Book, Box<dyn std::error::Error>>;
}

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

pub(crate) struct AtAGlanceReadOnlyMetadata<'a> {
    title: &'a str,
    author: &'a str,
    series: &'a str,
    isbn: &'a str,
    publisher: &'a str,
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

    pub(crate) fn get_at_a_glance_metadata(&self) -> AtAGlanceReadOnlyMetadata {
        AtAGlanceReadOnlyMetadata {
            title: self.title.as_ref(),
            author: self.author.as_deref().unwrap_or_default(),
            series: self.series.as_deref().unwrap_or_default(),
            isbn: self.isbn.as_deref().unwrap_or_default(),
            publisher: self.publisher.as_deref().unwrap_or_default(),
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
