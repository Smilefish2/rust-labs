use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:hello/comment";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/hello/comment.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**

注释对任何程序都不可缺少，同样 Rust 支持几种不同的注释方式。

普通注释，其内容将被编译器忽略掉：
// 单行注释，注释内容直到行尾。
/* 块注释， 注释内容一直到结束分隔符。 */
文档注释，其内容将被解析成 HTML 帮助文档:
/// 为接下来的项生成帮助文档。
//! 为注释所属于的项（译注：如 crate、模块或函数）生成帮助文档。

**/

pub fn sub_handler(_matches :&ArgMatches){
    // 这是行注释的例子
    // 注意有两个斜线在本行的开头
    // 在这里面的所有内容都不会被编译器读取

    println!("Hello, world!");

    // 请运行一下，你看到结果了吗？现在请将上述语句的两条斜线删掉，并重新运行。

    /*
     * 这是另外一种注释——块注释。一般而言，行注释是推荐的注释格式，
     * 不过块注释在临时注释大块代码特别有用。/* 块注释可以 /* 嵌套, */ */
     * 所以只需很少按键就可注释掉这些 main() 函数中的行。/*/*/* 自己试试！*/*/*/
     */

    /*
     注意，上面的例子中纵向都有 `*`，这只是一种风格，实际上这并不是必须的。
     */

    // 观察块注释是如何简单地对表达式进行修改的，行注释则不能这样。
    // 删除注释分隔符将会改变结果。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}