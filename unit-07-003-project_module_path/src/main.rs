/* 饮用模块项目的路径 */

// 在 Rust 模块树中找到一个项的位置，使用路径的方法。
//     绝对路径(absolte path) 是以 crate 根(root)开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，
//         对于当前 crate 的代码，则以字面值 crate 开头。
//     相对路径(relative path) 从当前模块开始，以 self、super 或定义在当前模块中的标识符开头。
// 绝对路径和相对路径都后跟一个或多个由双冒号(::) 分割的标识符。

// 在 Rust 中，默认所有的项(函数、方法、结构体、枚举、模块和常量)对父模块都是私有的。
// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用父模块中的项。
// 这是因为子模块封装并隐藏了它们的实现详情，但是子模块可以看到他们定义的上下文。
// Rust 提供了通过使用 pub 关键字来创建爱你公共项，使子模块的内部部分暴露给上级模块。

// 使模块公有并不能使模块内项也共有。模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码。
// 模块是一个容器，只是将模块变为公有能做的其实并不多；同时需要更深入的将一个或多个项变为公有。
// 私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。

fn main() {
    eat_at_restaurant();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {

        }
    }
}

fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
