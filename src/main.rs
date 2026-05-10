pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
pub(crate) mod parsers;
pub(crate) mod rendering;

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

    let all_book_paths_and_extensions = scan_sources_for_books().unwrap();
    let mut all_books: Vec<Book> = Vec::new();

    // TODO: Either change this for loop to use parallelism - currently parsing time increases as
    // number of books increase. We need to employ parallelism to solve this.
    // Archive extraction - I/O bound - use tokio
    // Parsing data extraction - CPU bound - use rayon
    for (book_path, book_type) in all_book_paths_and_extensions {
        match book_type {
            BookFileTypes::EpubFileType => {
                let mut epub = RawEpub::new(&book_path);
                all_books.push(epub.parse()?);
            }
            _ => {}
        }
    }
    let mut engine = RatatuiEngine;
    let mut app: RatatuiApp = engine.render::<BasicLayout, BasicPagination>(&all_books)?;
    app.run()?;
    Ok(())
}
