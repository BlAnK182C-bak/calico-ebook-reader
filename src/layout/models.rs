use crate::common::models::book::Book;
use crate::common::models::line::Line;

pub(crate) trait LayoutEngine {
    type OutputLayout;
    fn create_layout(max_width: usize, book: Book) -> Self::OutputLayout;
    fn get_all_sections(&self) -> &Vec<LayoutSection>;
}

#[derive(Debug)]
pub(crate) struct LayoutSection {
    id: String,
    lines: Vec<Line>,
}

impl LayoutSection {
    pub(crate) fn new(id: String, lines: Vec<Line>) -> Self {
        Self { id, lines }
    }

    pub(crate) fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}
