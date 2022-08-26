// pub mod deps;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:attribute/crate";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/attribute/crate.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
crate_type 属性可以告知编译器 crate 是一个二进制的可执行文件还是一个 库（甚至是哪种类型的库），crate_name 属性可以设定 crate 的名称。

不过，一定要注意在使用 cargo 时，这两种类型时都没有作用。由于大多数 Rust 工程都使用 cargo，这意味着 crate_type 和 crate_name 的作用事实上很有限。
 */

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
pub fn sub_handler(_matches :&ArgMatches){
    // 这个 crate 是一个库文件
    #![crate_type = "lib"]
    // 库的名称为 “rary”
    #![crate_name = "rary"]

    pub fn public_function() {
        println!("called rary's `public_function()`");
    }

    fn private_function() {
        println!("called rary's `private_function()`");
    }

    pub fn indirect_access() {
        print!("called rary's `indirect_access()`, that\n> ");

        private_function();
    }

    // 当用到 crate_type 属性时，就不再需要给 rustc 命令加上 --crate-type 标记。

    println!("参见：{}", "https://rustwiki.org/zh-CN/rust-by-example/attribute/crate.html");
}