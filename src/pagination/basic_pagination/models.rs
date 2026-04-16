use crate::{
    layout::models::LayoutEngine,
    pagination::models::{Page, PaginationEngine},
};

pub(crate) struct BasicPagination;

impl<L: LayoutEngine> PaginationEngine<L> for BasicPagination {
    type OutputPages = Vec<Page>;
    fn create_pages(layout: L, page_size: usize) -> Self::OutputPages {
        let sections = layout.get_all_sections();
        let mut pg_number: usize = 0;
        let mut all_pages: Vec<Page> = Vec::new();

        for s in sections.iter() {
            let lines = s.get_lines();
            for chunk in lines.chunks(page_size) {
                pg_number += 1;
                let page = Page::new(pg_number, chunk.to_vec());
                all_pages.push(page);
            }
        }
        all_pages
    }
}
