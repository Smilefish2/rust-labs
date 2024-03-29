use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std_misc/arg/matching";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/arg/matching.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
可以用模式匹配来解析简单的参数：
 */

use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

pub fn sub_handler(_matches :&ArgMatches){
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 没有传入参数
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // 一个传入参数
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // 传入一条命令和一个参数
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // 解析数字
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // 解析命令
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help();
                },
            }
        },
        // 所有其他情况
        _ => {
            // 显示帮助信息
            help();
        }
    }


    // $ ./match_args Rust
    // This is not the answer.
    // $ ./match_args 42
    // This is the answer!
    // $ ./match_args do something
    // error: second argument not an integer
    // usage:
    //     match_args <string>
    // Check whether given string is the answer.
    //     match_args {increase|decrease} <integer>
    //     Increase or decrease given integer by one.
    // $ ./match_args do 42
    // error: invalid command
    // usage:
    //     match_args <string>
    // Check whether given string is the answer.
    //     match_args {increase|decrease} <integer>
    //     Increase or decrease given integer by one.
    // $ ./match_args increase 42
    // 43
}