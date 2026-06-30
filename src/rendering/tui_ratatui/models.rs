use std::{collections::HashMap, io::Stdout};

use ratatui::{Terminal, prelude::CrosstermBackend, widgets::ListState};

use crate::{common::models::book::Book, pagination::models::Page, rendering::models::AppState};

pub(crate) struct RatatuiApp<'a> {
    pub(super) backend: Terminal<CrosstermBackend<Stdout>>,
    pub(super) state: AppState,

    pub(super) books: &'a [Book],
    pub(super) curr_book_pages: Option<Vec<Page>>,
    pub(super) curr_book_lookup: Option<HashMap<usize, usize>>,
    pub(super) list_state: ListState,

    pub(super) byte_offset: usize,
    pub(super) should_quit: bool,
}

pub(crate) struct RatatuiEngine;
