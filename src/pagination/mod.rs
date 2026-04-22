use crate::{layout::models::LayoutEngine, pagination::models::PaginationEngine};

pub(crate) mod basic_pagination;
pub(crate) mod models;
pub(crate) mod utils;

pub(crate) fn paginate<L: LayoutEngine, E: PaginationEngine<L>>(
    layout: L::OutputLayout,
    page_size: usize,
) -> E::OutputPages {
    E::create_pages(&layout, page_size)
}
