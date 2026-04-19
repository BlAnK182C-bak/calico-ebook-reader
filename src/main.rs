use crate::common::utils::settings::scan_sources_for_books;

pub(crate) mod common;
pub(crate) mod layout;
pub(crate) mod onboarding;
pub(crate) mod pagination;
pub(crate) mod parsers;
pub(crate) mod rendering;

fn main() {
    //TODO: Anywhere &String is being returned in a getter it needs to be changed to &str
    println!("Hello from Calico!");

    println!("Starting onboarding pipeline...");
    onboarding::pipeline();
    println!("Onboarding pipeline finished running successfully!");

    println!("{:#?}", scan_sources_for_books().unwrap());
}
