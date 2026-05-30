use super::models::BasicLayout;
use crate::common::models::book::Book;
use crate::common::models::line::Line;
use crate::layout::basic_layout::utils::wrap_words_to_next_line;
use crate::layout::models::{LayoutEngine, LayoutOutput, LayoutSection};

impl LayoutEngine for BasicLayout {
    type OutputLayout = BasicLayout;
    fn create_layout(max_width: usize, book: &Book) -> Self::OutputLayout {
        let mut base_offset: usize = 0;
        let sections = book
            .get_all_sections() // see below
            .iter()
            .map(|section| {
                let lines: Vec<Line> = section
                    .get_content()
                    .split("\n")
                    .flat_map(|l| {
                        let wrapped = wrap_words_to_next_line(l, max_width, base_offset);
                        // TODO: Investigate whether + 1 actually will work in all cases - what
                        // about things like \r\n that happen in windows
                        base_offset += l.len() + 1; // line length + whitespace 
                        wrapped
                    })
                    .collect();
                LayoutSection::new(String::from(section.get_id()), lines)
            })
            .collect();
        BasicLayout::new(sections)
    }
}

impl LayoutOutput for BasicLayout {
    fn get_all_sections(&self) -> &Vec<LayoutSection> {
        &self.sections
    }
}

impl BasicLayout {
    pub(crate) fn new(sections: Vec<LayoutSection>) -> Self {
        Self { sections }
    }
}

#[cfg(test)]
mod create_layout_basic_layout_tests {
    // AI generated tests

    use super::*;
    use crate::{
        common::utils::tests::{create_book, create_book_section},
        layout::models::LayoutOutput,
    };

    #[test]
    fn creates_empty_layout_when_book_has_no_sections() {
        let book = create_book(vec![]);

        let layout = BasicLayout::create_layout(10, &book);

        assert!(layout.get_all_sections().is_empty());
    }

    #[test]
    fn creates_single_section_layout() {
        let book = create_book(vec![create_book_section("intro", "hello world")]);

        let layout = BasicLayout::create_layout(20, &book);

        let sections = layout.get_all_sections();

        assert_eq!(sections.len(), 1);

        let lines = sections[0].get_lines();

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].get_line_content(), "hello world");
        assert_eq!(lines[0].get_offset_of_first_char(), 0);
    }

    #[test]
    fn wraps_long_lines_based_on_max_width() {
        let book = create_book(vec![create_book_section("intro", "hello world from rust")]);

        let layout = BasicLayout::create_layout(11, &book);

        let sections = layout.get_all_sections();

        let lines = sections[0].get_lines();

        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0].get_line_content(), "hello world");
        assert_eq!(lines[1].get_line_content(), "from rust");

        assert_eq!(lines[0].get_offset_of_first_char(), 0);
        assert_eq!(lines[1].get_offset_of_first_char(), 12);
    }

    #[test]
    fn preserves_offsets_across_newlines() {
        let book = create_book(vec![create_book_section("intro", "hello\nworld")]);

        let layout = BasicLayout::create_layout(20, &book);

        let sections = layout.get_all_sections();

        let lines = sections[0].get_lines();

        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0].get_line_content(), "hello");
        assert_eq!(lines[0].get_offset_of_first_char(), 0);

        assert_eq!(lines[1].get_line_content(), "world");
        assert_eq!(lines[1].get_offset_of_first_char(), 6);
    }

    #[test]
    fn preserves_offsets_across_multiple_sections() {
        let book = create_book(vec![
            create_book_section("s1", "hello"),
            create_book_section("s2", "world"),
        ]);

        let layout = BasicLayout::create_layout(20, &book);

        let sections = layout.get_all_sections();

        assert_eq!(sections.len(), 2);

        let first_lines = sections[0].get_lines();
        let second_lines = sections[1].get_lines();

        assert_eq!(first_lines[0].get_offset_of_first_char(), 0);
        assert_eq!(second_lines[0].get_offset_of_first_char(), 6);
    }

    #[test]
    fn handles_multiple_wrapped_lines_and_newlines() {
        let book = create_book(vec![create_book_section(
            "intro",
            "hello world from rust\nanother long line here",
        )]);

        let layout = BasicLayout::create_layout(11, &book);

        let sections = layout.get_all_sections();

        let lines = sections[0].get_lines();

        assert_eq!(lines.len(), 5);

        assert_eq!(lines[0].get_line_content(), "hello world");
        assert_eq!(lines[1].get_line_content(), "from rust");
        assert_eq!(lines[2].get_line_content(), "another");
        assert_eq!(lines[3].get_line_content(), "long line");
    }
}
