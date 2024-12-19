/* 定义模块来控制作用域与私有性 */

// 模块小抄
// 1. 从 crate 根节点开始：当编译一个 crate, 编译器首先在 crate 本文件(通常，对于一个库 crate 而言是 src/lib.rs, 对于一个二进制 crate 而言是 src/main.rs)中寻找需要被编译的代码
// 2. 声明模块：在 crate 跟文件中可以声明一个新模块；比如，mod garden; 声明一个 graden 模块。编译器会在下列路径中寻找模块代码：
//    · 内联，在大括号中，当 mod garden 后方不是一个分号而是一个大括号
//    · 在文件 src/garden.rs
//    · 在文件 src/garden/mod.rs
// 3. 声明子模块：在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在 src/garden.rs 中定义了 mod vegetables;。
//      编译器会在父模块命名目录中，寻找子模块代码：
//    · 内联，在大括号中，当 mod vegetables 后方不是一个分号而是一个大括号
//    · 在文件 src/garden/vegetables.rs
//    · 在文件 src/garden/vegetables/mod.rs
// 4. 模块中的代码路径：一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。
//      距离而言，一个 graden/vegetables 模块下 Asparagus 类型可以在 crate::garden::vegetables::Asparagus 被找到
// 5. 私有 VS 共有：一个模块里的代码默认对其父模块私有。为了使一个模块公用，应该使用 pub mod 代替 mod。为了使一个公用模块内成员公用，应该在声明前使用 pub。
// 6. use 关键字：在一个作用域内，use 关键字创建了一个成员的快捷方式，用来减少路径的重复。在任何可以引用 crate::garden::vegetables::Asparagus 的作用域，
//      你可以通过 use crate::garden::vegetables::Asparagus; 创建一个快捷方式，然后你就可以在作用域中写 Asparagus 来使用该类型。

use crate::garden::vegetables::Asparagus;

pub mod garden;
fn main() {
    println!("Hello, world!");
}
