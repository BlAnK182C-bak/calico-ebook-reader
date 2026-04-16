#[derive(Debug)]
pub(crate) struct Line {
    line_content: String,
}

impl Line {
    pub(crate) fn new(content: String) -> Self {
        Self {
            line_content: content,
        }
    }

    pub(crate) fn get_line_content(&self) -> &String {
        &self.line_content
    }
}
