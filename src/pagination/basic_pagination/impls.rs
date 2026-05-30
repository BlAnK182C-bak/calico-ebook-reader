use super::models::BasicPagination;
use crate::{
    layout::models::{LayoutEngine, LayoutOutput},
    pagination::models::{Page, PaginationEngine},
};

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

#[cfg(test)]
mod basic_pagination_tests {
    // AI generated tests

    use super::*;
    use crate::{
        common::models::{book::Book, line::Line},
        layout::models::{LayoutEngine, LayoutOutput, LayoutSection},
        pagination::models::PaginationEngine,
    };

    struct MockLayoutEngine;

    impl LayoutEngine for MockLayoutEngine {
        type OutputLayout = MockLayout;

        fn create_layout(_max_width: usize, _book: &Book) -> Self::OutputLayout {
            todo!()
        }
    }

    struct MockLayout {
        sections: Vec<LayoutSection>,
    }

    impl LayoutOutput for MockLayout {
        fn get_all_sections(&self) -> &Vec<LayoutSection> {
            &self.sections
        }
    }

    fn create_line(content: &str, offset: usize) -> Line {
        Line::new(&content.to_string(), offset)
    }

    fn create_section(lines: Vec<Line>) -> LayoutSection {
        LayoutSection::new("section-1".to_string(), lines)
    }

    fn create_layout(sections: Vec<LayoutSection>) -> MockLayout {
        MockLayout { sections }
    }

    #[test]
    fn returns_empty_pages_when_layout_has_no_sections() {
        let layout = create_layout(vec![]);

        let result =
            <BasicPagination as PaginationEngine<MockLayoutEngine>>::create_pages(&layout, 2);

        assert!(result.is_empty());
    }

    #[test]
    fn creates_single_page_when_lines_fit_in_page_size() {
        let layout = create_layout(vec![create_section(vec![
            create_line("hello", 0),
            create_line("world", 6),
        ])]);

        let result =
            <BasicPagination as PaginationEngine<MockLayoutEngine>>::create_pages(&layout, 10);

        assert_eq!(result.len(), 1);

        let page = &result[0];

        assert_eq!(page.get_start_offset(), 0);
        assert_eq!(page.get_content().len(), 2);
    }

    #[test]
    fn splits_lines_into_multiple_pages() {
        let layout = create_layout(vec![create_section(vec![
            create_line("a", 0),
            create_line("b", 2),
            create_line("c", 4),
            create_line("d", 6),
            create_line("e", 8),
        ])]);

        let result =
            <BasicPagination as PaginationEngine<MockLayoutEngine>>::create_pages(&layout, 2);

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].get_start_offset(), 0);
        assert_eq!(result[1].get_start_offset(), 4);
        assert_eq!(result[2].get_start_offset(), 8);

        assert_eq!(result[0].get_content().len(), 2);
        assert_eq!(result[1].get_content().len(), 2);
        assert_eq!(result[2].get_content().len(), 1);
    }

    #[test]
    fn creates_pages_across_multiple_sections() {
        let layout = create_layout(vec![
            create_section(vec![create_line("a", 0), create_line("b", 2)]),
            create_section(vec![create_line("c", 10), create_line("d", 12)]),
        ]);

        let result =
            <BasicPagination as PaginationEngine<MockLayoutEngine>>::create_pages(&layout, 1);

        assert_eq!(result.len(), 4);

        assert_eq!(result[0].get_start_offset(), 0);
        assert_eq!(result[1].get_start_offset(), 2);
        assert_eq!(result[2].get_start_offset(), 10);
        assert_eq!(result[3].get_start_offset(), 12);
    }
}
