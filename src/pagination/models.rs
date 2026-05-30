use crate::common::models::line::Line;
use crate::layout::models::LayoutEngine;

pub(crate) trait PaginationEngine<L: LayoutEngine> {
    type OutputPages;
    fn create_pages(layout: &L::OutputLayout, page_size: usize) -> Self::OutputPages;
}

#[allow(dead_code)]
pub(crate) struct Page {
    pub(super) content: Vec<Line>,
    pub(super) start_byte_offset: usize,
    pub(super) end_byte_offset: usize,
}
