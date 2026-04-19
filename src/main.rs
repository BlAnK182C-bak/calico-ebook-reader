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

pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
pub(crate) mod parsers;
pub(crate) mod rendering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: Anywhere &String is being returned in a getter it needs to be changed to &str
    println!("Hello from Calico!");

    println!("Starting onboarding pipeline...");
    onboarding::pipeline();
    println!("Onboarding pipeline finished running successfully!");

    let all_book_paths_and_extensions = scan_sources_for_books().unwrap();
    let mut all_books: Vec<Book> = Vec::new();

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
