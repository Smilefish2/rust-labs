use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:flow_control/match/destructuring/destructure_tuple";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring/destructure_tuple.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
元组
    元组可以在 match 中解构，如下所示：
 */
pub fn sub_handler(_matches :&ArgMatches){
    let pair = (0, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        // 解构出第二个值
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}