pub mod deps;
pub mod conventions;
pub mod test;
pub mod build_scripts;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:cargo";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/cargo.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
cargo 是官方的 Rust 包管理工具。 它有很多非常有用的功能来提高代码质量和开发人员的开发效率！ 这些功能包括：

    依赖管理和与 crates.io（官方 Rust 包注册服务）集成
    方便的单元测试
    方便的基准测试
    本章将介绍一些快速入门的基础知识，不过你可以在 cargo 官方手册中找到详细内容。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("{}", "cargo 是官方的 Rust 包管理工具。 它有很多非常有用的功能来提高代码质量和开发人员的开发效率");
}