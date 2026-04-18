use crate::common::models::book::Book;
use crate::common::models::line::Line;
use crate::layout::basic_layout::utils::wrap_words_to_next_line;
use crate::layout::models::{LayoutEngine, LayoutOutput, LayoutSection};

#[derive(Debug)]
pub(crate) struct BasicLayout {
    sections: Vec<LayoutSection>,
}

impl BasicLayout {
    pub(crate) fn new(sections: Vec<LayoutSection>) -> Self {
        Self { sections }
    }
}
impl LayoutEngine for BasicLayout {
    type OutputLayout = BasicLayout;
    fn create_layout(max_width: usize, book: Book) -> Self::OutputLayout {
        let sections = book
            .get_all_sections() // see below
            .iter()
            .map(|section| {
                let lines: Vec<Line> = section
                    .get_content()
                    .split("\n")
                    .flat_map(|l| wrap_words_to_next_line(l, max_width))
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
