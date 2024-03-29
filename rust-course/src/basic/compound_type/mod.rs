pub mod string_slice;
pub mod tuple;
pub mod r#struct;
pub mod r#enum;
pub mod array;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/compound_type";
const ABOUT: &str = "https://course.rs/basic/compound-type/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}", "https://course.rs/basic/compound-type/intro.html");
}