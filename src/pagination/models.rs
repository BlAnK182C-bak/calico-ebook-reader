use crate::common::models::line::Line;
use crate::layout::models::LayoutEngine;

pub(crate) trait PaginationEngine<L: LayoutEngine> {
    type OutputPages;
    fn create_pages(layout: &L::OutputLayout, page_size: usize) -> Self::OutputPages;
}

pub(crate) struct Page {
    pub(crate) content: Vec<Line>,
    pub(crate) start_byte_offset: usize,
    pub(crate) end_byte_offset: usize,
}
