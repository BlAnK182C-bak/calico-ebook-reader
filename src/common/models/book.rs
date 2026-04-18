use super::filetypes::BookFileTypes;

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct BookSection {
    id: String,
    name: Option<String>,
    content: String,
}

#[derive(Clone)]
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
    pub(crate) fn get_id(&self) -> &str {
        &self.id
    }
    pub(crate) fn get_content(&self) -> &str {
        &self.content
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

    pub(crate) fn get_all_sections(&self) -> &Vec<BookSection> {
        &self.content
    }

    pub(crate) fn get_title(&self) -> &String {
        &self.metadata.title
    }
}
