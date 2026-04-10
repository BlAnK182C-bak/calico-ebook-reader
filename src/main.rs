use crate::parsers::models::ParserEngine;

pub(crate) mod layout;
pub(crate) mod misc;
pub(crate) mod onboarding;
pub(crate) mod parsers;

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

    println!("{:#?}", pj1);
    println!("{:#?}", hp1);
}
