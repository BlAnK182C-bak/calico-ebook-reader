pub(crate) enum BookFileTypes {
    EpubFileType,
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

pub(crate) struct BookSections {
    id: String,
    name: Option<String>,
    content: String,
}

pub(crate) struct Book {
    metadata: BookMetadata,
    file_type: BookFileTypes,
    content: Vec<BookSections>,
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
