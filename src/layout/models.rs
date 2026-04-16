use crate::common::models::book::Book;
use crate::common::models::line::Line;

pub(crate) trait Layout {
    fn get_lines(&self) -> &Vec<Line>;
    fn display_all_lines(&self);
}

#[derive(Debug)]
pub(crate) struct LayoutSection {
    id: String,
    lines: Vec<Line>,
}

pub(crate) trait LayoutEngine {
    type OutputLayout;
    fn create_layout(max_width: usize, book: Book) -> Self::OutputLayout;
}

impl LayoutSection {
    pub(crate) fn new(id: String, lines: Vec<Line>) -> Self {
        Self { id, lines }
    }
}
