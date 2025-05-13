/* Trait: 定义共同行为 */
// trait 定义了某个特定类型拥有与其他类型共享的功能
// 可以通过 trait 以一种抽象的方式定义共同行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型
// trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合
// 是不能为外部类型实现外部 trait。例如，不能在 aggregator  crate 中为 Vec<T>  实现 Display  trait。
//  这是因为 Display  和 Vec<T>  都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。
//  这个限制是被称为 相干性（coherence）的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。
//  这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。
fn main() {
    println!("Hello, world!");
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> string {
        format!("{}: {}", self.username, self.content)
    }
}


// 默认实现 trait
// 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自
//  己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
// 重载一个默认实现的语法与实现没有默认实现的 trait 方法的语法一样。
// pub trait Summary {
//     fn summarize_author(&self) -> String {
//         String::from("author")
//     }
// }
// impl Summary for NewsArticle {}


// Trait 作为参数
// 使用 trait 作为参数，来接受多种不同类型的参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}