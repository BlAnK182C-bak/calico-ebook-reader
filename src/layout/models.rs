use crate::common::models::book::Book;
use crate::common::models::line::Line;

pub(crate) trait LayoutOutput {
    fn get_all_sections(&self) -> &Vec<LayoutSection>;
}

pub(crate) trait LayoutEngine {
    type OutputLayout: LayoutOutput;
    fn create_layout(max_width: usize, book: &Book) -> Self::OutputLayout;
}

pub(crate) struct LayoutSection {
    pub(super) id: String,
    pub(super) lines: Vec<Line>,
}
