use crate::layout::basic_layout::models::BasicLayout;
use crate::pagination::basic_pagination::models::BasicPagination;
use crate::parsers::models::ParserEngine;
use crate::rendering::models::{RenderApp, RenderingEngine};
use crate::rendering::tui_ratatui::models::RatatuiEngine;

pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
pub(crate) mod parsers;
pub(crate) mod rendering;

fn main() {
    println!("Hello from Calico!");

    println!("Starting onboarding pipeline...");
    onboarding::pipeline();
    println!("Onboarding pipeline finished running successfully!");

    // TODO: These will be removed come actual app - because ofc.
    let mut pj1_epub = parsers::epub::models::RawEpub::new(
        "/home/abhinavks/Downloads/01_The_Lightning_Thief.epub",
    );
    let mut hp1_epub = parsers::epub::models::RawEpub::new(
        "/home/abhinavks/Downloads/01_Harry_Potter_and_the_Sorcerer_39_s_Stone_-_J_K_Rowling.epub",
    );

    let pj1 = pj1_epub.parse().unwrap();
    let hp1 = hp1_epub.parse().unwrap();

    // TODO: Code cleaning I don't like that I am calling engine (which is &mut self) as a param due
    // to generic types.
    let mut engine = RatatuiEngine;
    let mut app = <RatatuiEngine as RenderingEngine<BasicLayout, BasicPagination>>::render(
        &mut engine,
        pj1.clone(),
    )
    .unwrap();

    app.run("The lightning Thief").unwrap();
}
