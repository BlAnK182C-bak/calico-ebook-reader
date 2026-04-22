use super::filetypes::BookFileTypes;

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

pub(crate) struct BookSection {
    id: String,
    name: Option<String>,
    content: String,
}

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

    pub(crate) fn get_title(&self) -> String {
        format!(
            "{} by {}",
            self.metadata.title,
            self.metadata.author.as_deref().unwrap_or("Unknown Author")
        )
    }

    pub(crate) fn get_metadata(&self) -> String {
        format!(
            "{} by {}\n\n {}\n\n Series: #{} of {} \n\n Subjects: {}\n\n Rights: {} | {} | {}",
            self.metadata.title,
            self.metadata.author.as_deref().unwrap_or("Unknown Author"),
            self.metadata.description.as_deref().unwrap_or("-"),
            self.metadata
                .series_order_number
                .map(|n| n.to_string())
                .unwrap_or(String::from("-1")),
            self.metadata.series.as_deref().unwrap_or("N/A"),
            self.metadata
                .subjects
                .as_ref()
                .map(|s| s.join(", "))
                .unwrap_or(String::from("N/A")),
            self.metadata
                .publisher
                .as_deref()
                .unwrap_or("Unknown Publisher"),
            self.metadata.rights.as_deref().unwrap_or("Unknown Rights"),
            self.metadata.isbn.as_deref().unwrap_or("Unknown ISBN")
        )
    }

    pub(crate) fn get_id(&self) -> String {
        String::from(format!(
            "{}|{}|{}",
            self.metadata.title,
            self.metadata
                .author
                .as_ref()
                .unwrap_or(&String::from("UNKN")),
            self.metadata.isbn.as_ref().unwrap_or(&String::from("ISBN"))
        ))
    }
}
