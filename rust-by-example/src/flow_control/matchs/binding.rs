use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:flow_control/match/binding";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/binding.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match 提供了 @ 符号来绑定变量到名称：
 */
// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("Tell me type of person you are");

    match age() {
        0             => println!("I'm not born yet I guess"),
        // 可以直接 `match` 1 ..= 12，但怎么把岁数打印出来呢？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }

    // 你也可以使用绑定来“结构” enum 变体，例如 Option:
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }
}