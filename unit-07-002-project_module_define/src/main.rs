/* 定义模块来控制作用域与私有性 */

// 模块小抄
// 1. 从 crate 根节点开始：当编译一个 crate, 编译器首先在 crate 本文件(通常，对于一个库 crate 而言是 src/lib.rs, 对于一个二进制 crate 而言是 src/main.rs)中寻找需要被编译的代码
// 2. 声明模块：在 crate 跟文件中可以声明一个新模块；比如，mod garden;声明一个 graden 模块。编译器会在下列路径中寻找模块代码：
fn main() {
    println!("Hello, world!");
}
