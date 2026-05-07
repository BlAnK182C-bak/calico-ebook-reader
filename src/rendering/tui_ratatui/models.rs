use std::{collections::HashMap, io::Stdout};

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{common::models::book::Book, pagination::models::Page, rendering::models::AppState};

pub(crate) struct RatatuiApp<'a> {
    pub(crate) backend: Terminal<CrosstermBackend<Stdout>>,
    pub(crate) state: AppState,

    pub(crate) books: &'a Vec<Book>,
    pub(crate) curr_book_pages: Option<Vec<Page>>,
    pub(crate) curr_book_lookup: Option<HashMap<usize, usize>>,
    pub(crate) curr_book_idx: usize,

    pub(crate) byte_offset: usize,
    pub(crate) should_quit: bool,
}

pub(crate) struct RatatuiEngine;
