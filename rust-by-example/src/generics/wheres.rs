use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:generics/where";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/generics/where.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
where 分句
    约束也可以使用 where 分句来表达，它放在 { 的前面，而不需写在类型第一次出现 之前。另外 where 从句可以用于任意类型的限定，而不局限于类型参数本身。

    where 在下面一些情况下有很用：

当分别指定泛型的类型和约束会更清晰时：
    impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

    // 使用 `where` 从句来表达约束
    impl <A, D> MyTrait<A, D> for YourType where
        A: TraitB + TraitC,
        D: TraitE + TraitF {}

    当使用 where 从句比正常语法更有表现力时。本例中的 impl 如果不用 where 从句，就无法直接表达。
 */

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
// 或着改用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}