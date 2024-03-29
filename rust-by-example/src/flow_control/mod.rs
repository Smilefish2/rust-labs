use clap::{App, ArgMatches};

pub mod if_else;
pub mod loops;
pub mod whiles;
pub mod fors;
pub mod matchs;
pub mod if_let;
pub mod while_let;

pub const NAME: &str = "rust-by-example:flow_control";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("任何编程语言都包含的一个必要部分就是改变控制流程：if/else，for 等。让我们 谈谈 Rust 语言中的这部分内容。");
}