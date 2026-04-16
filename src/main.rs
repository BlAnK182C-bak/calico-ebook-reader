use crate::layout::basic_layout::models::BasicLayout;
use crate::layout::layoutize;
use crate::pagination::basic_pagination::models::BasicPagination;
use crate::pagination::paginate;
use crate::parsers::models::ParserEngine;

pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
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

    let layoutized_pj1 = layoutize::<BasicLayout>(pj1, 500);
    let layoutized_hp1 = layoutize::<BasicLayout>(hp1, 500);

    let paginated_pj1 = paginate::<BasicLayout, BasicPagination>(layoutized_pj1, 10);
    let paginated_hp1 = paginate::<BasicLayout, BasicPagination>(layoutized_hp1, 10);

    println!("{:#?}", paginated_hp1);
    println!("{:#?}", paginated_pj1);
}
