use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:trait/drop";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/trait/drop.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该 方法。Drop trait 的主要作用是释放实现者的实例拥有的资源。

Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放 资源的类型。Drop trait 也可以为任何自定义数据类型手动实现。

下面示例给 drop 函数增加了打印到控制台的功能，用于宣布它在什么时候被调用。 when it is called.）
*/

struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了打印到控制台的功能。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let _a = Droppable { name: "a" };

    // 代码块 A
    {
        let _b = Droppable { name: "b" };

        // 代码块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // 变量可以手动使用 `drop` 函数来销毁。
    drop(_a);
    // 试一试 ^ 将此行注释掉。

    println!("end of the main function");

    // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁。
}