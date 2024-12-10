// 方法语法

// 方法(method) 与 函数类似：他们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时执行的代码。
// 也有与函数不同的地方: 方法在结构体上下文中被定义（或者是 枚举或 trait 对象的上下文），并且他们的第一个参数总是 self, 它代表调用该方法的结构体实例。
// 继续 unit-05-002-struct_example 工程中计算长方形面积的实现重构为例
fn main() {
    // struct_rectangles();
    can_hold_test();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//为了 使函数位于 Rectangle 的上下文中，我们开始了一个 impl 块，这个 impl 块中所有内容都将与 Rectangle 类型相关。
// 使用 &self 替代 rectangle: &Rectangle, &self 实际上是 self: &Self 的缩写。
// 在一个 impl 块中， Self 类型是 impl 块类型的别名。
// 方法的第一个参数必须有一个名为 self 的 Self 类型的参数。
// 注意：我们任然需要在 self 前使用 & 标识这个方法借用了 Self 实例。
// 方法可以选择获取 self 的所有权，或 不变的借用 self, 或者可变的借用 self。
// 通过仅用 self 作为第一参数来使方法获取实例的所有权是很少见的；
//这种方法通常用在当方法将 self 转换为别的实例的时候，这时我们我们想要防止调用者在转换后使用原始的实例。
// 使用方法替换函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型外，主要好处在于组织性。
// 我们将某个实例能做的所有事情都放放入 impl 中，而不是让将来的用户在我们的库中导出寻找 实例 的功能。
// 我们可以选择将方法的名称与结构中的一个字段相同。
// 与字段同名方法将被定义为值返回字段中的值，而不做其他事情。这样的方法被称为 getters
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数：所有又在 impl 定义的函数，因为它们与 impl 后命名的类型相关。
    // 关联函数经常被用作返回一个结构体新实力的构造函数。这些函数的名称通常为 new,但 new 不是关键字。
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    // // 定义是否包含函数
    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }
}

// 每个结构体可以拥有多个 impl 块。但是没必要将一个对象的方法分散到多个 impl 块中，不过这是有效的语法
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn can_hold_test() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 19, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));           // multiple `can_hold` found，因为两个 impl 块都有 can_hold 函数
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));   
}

// // 使用方法语法 (method syntax) 在 Recangle 实例上调用 area 方法。
// // 方法语法(method syntax): 获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。e.g.【rect.area()】
// fn struct_rectangles() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect.area();
//     );
// }



// // Rust 中有个：自动引用和解引用(automatic referencing and dereferencing)的功能
// // 方法调用是 Rust 中少数几个拥有这个行为的地方
// // 工作原理： 当使用 object.something() 调用方法是，Rust 自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
// fn automatic_reference() {
//     let p1 = Point { x: 0.0, y: 0.0 };
//     let p2 = Point { x: 1.0, y: 1.0 };

//     p1.distance(&p2);                               // 自动引用的行为之所以有效，是因为方法中有一个明确的接收者----- self 的类型
//     (&p1).distance(&p2);
// }

// #[derive(Debug, Clone, Copy)]
// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Point {
//     fn distance(&self, other:&Point) -> f64 {
//         let x_squared = f64::powi(other.x - self.x, 2);
//         let y_squared = f64::powi(other.y - self.y, 2);
//         f64::sqrt(x_squared + y_squared)
//     }
// }

