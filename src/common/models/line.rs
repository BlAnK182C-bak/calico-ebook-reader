#[derive(Debug, Clone)]
pub(crate) struct Line {
    line_content: String,
    offset_of_first_char: usize,
}

impl Line {
    pub(crate) fn new(content: &String, offset_of_first_char: usize) -> Self {
        Self {
            line_content: content.into(),
            offset_of_first_char,
        }
    }

    pub(crate) fn get_line_content(&self) -> &str {
        &self.line_content
    }

    pub(crate) fn get_offset_of_first_char(&self) -> usize {
        self.offset_of_first_char
    }
}
