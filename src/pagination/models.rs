use crate::common::models::line::Line;
use crate::layout::models::LayoutEngine;

pub(crate) trait PaginationEngine<L: LayoutEngine> {
    type OutputPages;
    fn create_pages(layout: &L::OutputLayout, page_size: usize) -> Self::OutputPages;
}

#[derive(Debug)]
pub(crate) struct Page {
    number: usize,
    content: Vec<Line>,
}

impl Page {
    pub(crate) fn new(number: usize, content: Vec<Line>) -> Self {
        Self { number, content }
    }

    pub(crate) fn get_content(&self) -> &Vec<Line> {
        &self.content
    }
}
