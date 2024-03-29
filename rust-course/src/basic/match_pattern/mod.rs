pub mod match_if_let;
pub mod option;
pub mod pattern_match;
pub mod all_patterns;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/match-pattern";
const ABOUT: &str = "https://course.rs/match-pattern/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("https://course.rs/basic/match-pattern/intro.html");
}