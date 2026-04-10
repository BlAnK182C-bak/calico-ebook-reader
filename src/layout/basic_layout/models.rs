use crate::{
    layout::models::{BookWithAppliedLayout, LayoutEngine, Line},
    parsers::models::Book,
};

pub(super) struct BasicLayout {
    all_lines: Vec<Line>,
}

impl Line {
    pub(super) fn new(content: String) -> Self {
        Self {
            line_content: content,
        }
    }

    pub(super) fn apply_layout(&self, max_width: usize) -> Vec<Self> {
        self.line_content
            .chars()
            .collect::<Vec<char>>()
            .chunks(max_width)
            .map(|chunk| Self {
                line_content: chunk.iter().collect(),
            })
            .collect()
    }
}

impl LayoutEngine for BasicLayout {
    fn create_layout(&self, max_width: usize, book: Book) -> BookWithAppliedLayout {
        todo!();
    }
}

impl BasicLayout {
    pub(super) fn new(&mut self, book: Book) -> Self {
        todo!();
    }
}
