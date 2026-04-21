use crate::{
    layout::models::{LayoutEngine, LayoutOutput},
    pagination::models::{Page, PaginationEngine},
};

pub(crate) struct BasicPagination;

impl<L: LayoutEngine> PaginationEngine<L> for BasicPagination {
    type OutputPages = Vec<Page>;
    fn create_pages(layout: &L::OutputLayout, page_size: usize) -> Self::OutputPages {
        let sections = layout.get_all_sections();
        let mut all_pages: Vec<Page> = Vec::new();

        for s in sections.iter() {
            let lines = s.get_lines();
            for chunk in lines.chunks(page_size) {
                let first_line = chunk.first().unwrap();
                let last_line = chunk.last().unwrap();

                let start_offset = first_line.get_offset_of_first_char();
                let end_offset =
                    last_line.get_offset_of_first_char() + last_line.get_line_content().len();

                let page = Page::new(chunk.to_vec(), start_offset, end_offset);
                all_pages.push(page);
            }
        }
        all_pages
    }
}
