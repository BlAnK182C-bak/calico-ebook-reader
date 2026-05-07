use super::models::LayoutSection;
use crate::common::models::line::Line;

impl LayoutSection {
    pub(crate) fn new(id: String, lines: Vec<Line>) -> Self {
        Self { id, lines }
    }

    pub(crate) fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}
