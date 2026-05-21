pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
pub(crate) mod parsers;
pub(crate) mod rendering;

use rayon::prelude::*;

use crate::{
    common::{
        models::{book::Book, filetypes::BookFileTypes},
        utils::settings::scan_sources_for_books,
    },
    layout::basic_layout::models::BasicLayout,
    pagination::basic_pagination::models::BasicPagination,
    parsers::{epub::models::RawEpub, models::ParserEngine},
    rendering::{
        models::{RenderApp, RenderingEngine},
        tui_ratatui::models::{RatatuiApp, RatatuiEngine},
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: When we separated impls and models, we made everything pub(super) - need to evaluate is
    // there a necessity even for getters and setters at this point - I am trying to inject a OOPS
    // concept into a functional language :"))

    println!("Hello from Calico!");

    println!("Starting onboarding pipeline...");
    onboarding::pipeline();
    println!("Onboarding pipeline finished running successfully!");

    let new_books_paths_and_extensions = scan_sources_for_books().unwrap();

    let all_books: Vec<Book> = new_books_paths_and_extensions
        .par_iter()
        .filter_map(|(book_path, book_type)| match book_type {
            BookFileTypes::EpubFileType => {
                let mut epub = RawEpub::new(&book_path);
                epub.parse().ok()
            }
            _ => None,
        })
        .collect();

    let mut engine = RatatuiEngine;
    let mut app: RatatuiApp = engine.render::<BasicLayout, BasicPagination>(&all_books)?;
    app.run()?;
    Ok(())
}
