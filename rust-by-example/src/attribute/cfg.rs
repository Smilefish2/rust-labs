pub mod custom;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:attribute/cfg";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/attribute/cfg.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
条件编译可能通过两种不同的操作符实现：

    cfg 属性：在属性位置中使用 #[cfg(...)]
    cfg! 宏：在布尔表达式中使用 cfg!(...)
两种形式使用的参数语法都相同。
 */

// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

pub fn sub_handler(_matches :&ArgMatches){
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    println!("参见：{}", "https://rustwiki.org/zh-CN/rust-by-example/attribute/crate.html");
}