use crate::common::models::line::Line;
use crate::layout::models::LayoutEngine;

pub(crate) trait PaginationEngine<L: LayoutEngine> {
    type OutputPages;
    fn create_pages(layout: &L::OutputLayout, page_size: usize) -> Self::OutputPages;
}

#[derive(Debug)]
pub(crate) struct Page {
    content: Vec<Line>,
    start_byte_offset: usize,
    end_byte_offset: usize,
}

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
