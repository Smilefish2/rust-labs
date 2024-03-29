pub mod explicit;
pub mod functions;
pub mod methods;
pub mod structs;
pub mod traits;
pub mod lifetime_bounds;
pub mod lifetime_coercion;
pub mod static_lifetime;
pub mod elision;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:scope/lifetime";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
生命周期（lifetime）是这样一种概念，编译器（中的借用检查器）用它来保证所有的 借用都是有效的。
确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候 结束。
虽然生命周期和作用域经常被一起提到，但它们并不相同。

例如考虑这种情况，我们通过 & 来借用一个变量。该借用拥有一个生命周期，此生命 周期由它声明的位置决定。
于是，只要该借用在出借者（lender）被销毁前结束，借用 就是有效的。
然而，借用的作用域则是由使用引用的位置决定的。

在下面的例子和本章节剩下的内容里，我们将看到生命周期和作用域的联系与区别。

译注：如果代码中的生命周期示意图乱掉了，请把它复制到任何编辑器中，用等宽字体 查看。
为避免中文的显示问题，下面一些注释没有翻译。
*/

// 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期。
// `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和
// `borrow2` 的。`borrow1` 和 `borrow2` 的周期没有关联，
// 因为它们各不相交
pub fn sub_handler(_matches :&ArgMatches){
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
