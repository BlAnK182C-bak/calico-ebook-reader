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
    content: String,
}

pub(crate) struct Book {
    content: Vec<BookSection>,
    book_title: String,
    readable_metadata: String,
    book_id: String,
}

#[allow(clippy::too_many_arguments)]
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
            title,
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
    pub(crate) fn new(id: String, content: String) -> Self {
        Self { id, content }
    }
    pub(crate) fn get_id(&self) -> &str {
        &self.id
    }
    pub(crate) fn get_content(&self) -> &str {
        &self.content
    }
}

impl Book {
    pub(crate) fn new(metadata: BookMetadata, content: Vec<BookSection>) -> Self {
        let book_title: String = format!(
            "{} by {}",
            &metadata.title,
            &metadata.author.as_deref().unwrap_or("Unknown Author")
        );

        let readable_metadata: String = format!(
            "{} by {}\n\n {}\n\n Series: #{} of {} \n\n Subjects: {}\n\n Rights: {} | {} | {}",
            &metadata.title,
            &metadata.author.as_deref().unwrap_or("Unknown Author"),
            &metadata.description.as_deref().unwrap_or("-"),
            &metadata
                .series_order_number
                .map(|n| n.to_string())
                .unwrap_or(String::from("-1")),
            &metadata.series.as_deref().unwrap_or("N/A"),
            &metadata
                .subjects
                .as_ref()
                .map(|s| s.join(", "))
                .unwrap_or(String::from("N/A")),
            &metadata.publisher.as_deref().unwrap_or("Unknown Publisher"),
            &metadata.rights.as_deref().unwrap_or("Unknown Rights"),
            &metadata.isbn.as_deref().unwrap_or("Unknown ISBN")
        );

        let book_id: String = format!(
            "{}|{}|{}",
            &metadata.title,
            &metadata.author.as_ref().unwrap_or(&String::from("UNKN")),
            &metadata.isbn.as_ref().unwrap_or(&String::from("ISBN"))
        );

        Self {
            content,
            book_title,
            readable_metadata,
            book_id,
        }
    }

    pub(crate) fn get_all_sections(&self) -> &Vec<BookSection> {
        &self.content
    }

    pub(crate) fn get_title(&self) -> &str {
        &self.book_title
    }

    pub(crate) fn get_metadata(&self) -> &str {
        &self.readable_metadata
    }

    pub(crate) fn get_id(&self) -> &str {
        &self.book_id
    }
}
