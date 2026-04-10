use crate::parsers::models::Book;

pub(crate) trait LayoutEngine {
    fn create_layout(&self, max_width: usize, book: Book) -> BookWithAppliedLayout;
}

pub(crate) struct Line {
    pub(crate) line_content: String,
}

pub(crate) struct Layout {
    all_content: Vec<Line>,
}

pub(crate) struct BookWithAppliedLayout {
    book: Book,
    layout: Layout,
}
