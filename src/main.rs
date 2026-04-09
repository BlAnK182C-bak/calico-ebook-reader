pub(crate) mod misc;
pub(crate) mod onboarding;
pub(crate) mod parsers;

fn main() {
    println!("Hello from Calico!");

    println!("Starting onboarding pipeline...");
    onboarding::pipeline();
    println!("Onboarding pipeline finished running successfully!");

    // TODO: These will be removed come actual app - because ofc.
    let pj1 =
        parsers::epub::epub_parse("/home/abhinavks/Downloads/01_The_Lightning_Thief.epub").unwrap();
    let hp1 = parsers::epub::epub_parse(
        "/home/abhinavks/Downloads/01_Harry_Potter_and_the_Sorcerer_39_s_Stone_-_J_K_Rowling.epub",
    )
    .unwrap();

    println!("{:#?}", pj1);
    println!("{:#?}", hp1);
}
