use super::models::Page;
use crate::common::models::line::Line;

impl Page {
    pub(crate) fn new(
        content: Vec<Line>,
        start_byte_offset: usize,
        end_byte_offset: usize,
    ) -> Self {
        Self {
            content,
            start_byte_offset,
            end_byte_offset,
        }
    }

    pub(crate) fn get_content(&self) -> &Vec<Line> {
        &self.content
    }

    pub(crate) fn get_start_offset(&self) -> usize {
        self.start_byte_offset
    }
}
