use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:scope/move/partial_move";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/scope/move/partial_move.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
当所有权转移时，数据的可变性可能发生改变。
 */
pub fn sub_handler(_matches :&ArgMatches){
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);
}