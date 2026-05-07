use std::{collections::HashMap, io::Stdout};

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{common::models::book::Book, pagination::models::Page, rendering::models::AppState};

pub(crate) struct RatatuiApp<'a> {
    pub(super) backend: Terminal<CrosstermBackend<Stdout>>,
    pub(super) state: AppState,

    pub(super) books: &'a Vec<Book>,
    pub(super) curr_book_pages: Option<Vec<Page>>,
    pub(super) curr_book_lookup: Option<HashMap<usize, usize>>,
    pub(super) curr_book_idx: usize,

    pub(super) byte_offset: usize,
    pub(super) should_quit: bool,
}

pub(crate) struct RatatuiEngine;
