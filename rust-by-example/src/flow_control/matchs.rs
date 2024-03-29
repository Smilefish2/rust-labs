use clap::{App, ArgMatches};

pub mod destructuring;
pub mod guard;
pub mod binding;

pub const NAME: &str = "rust-by-example:flow_control/match";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
Rust 通过 match 关键字来提供模式匹配，用法和 C 语言的的 switch 类似。
 */
pub fn sub_handler(_matches :&ArgMatches){
    let number = 13;
    // 试一试 ^ 将不同的值赋给 `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}