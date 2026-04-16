use crate::layout::models::LayoutEngine;

pub(crate) trait PaginationEngine<L: LayoutEngine> {
    type OutputPages;
    fn create_pages(layout: L) -> Self::OutputPages;
}
