// 方法语法

// 方法(method) 与 函数类似：他们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时执行的代码。
// 也有与函数不同的地方: 方法在结构体上下文中被定义（或者是 枚举或 trait 对象的上下文），并且他们的第一个参数总是 self, 它代表调用该方法的结构体实例。
// 继续 unit-05-002-struct_example 工程中计算长方形面积的实现重构为例
fn main() {
    struct_rectangles();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//为了 使函数位于 Rectangle 的上下文中，我们开始了一个 impl 块，这个 impl 块中所有内容都将与 Rectangle 类型相关。

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 使用方法语法 (method syntax) 在 Recangle 实例上调用 area 方法。
// 方法语法(method syntax): 获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。e.g.【rect.area()】
fn struct_rectangles() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

}