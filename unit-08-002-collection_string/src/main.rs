/* 使用字符串储存 UTF-8 编码的文本 */

use std::ops::Add;

// Rust 核心语言中只有一种字符串类型：字符串 slice str,它通常以被借用的形式出现 &str。
// 字符串 slice: 一些对存储在别处的 UTF-8 编码字符串数据的引用。
// 字符串(String)类型：由Rust标准库提供，而不是编入核心语言，是一种 可增长、可变、可拥有、UTF-8编码的字符串类型。
// 字符串 slice str 与 字符串(String)是不同的两种类型，且两种类型都是 UTF-8 编码的。
fn main() {
    // println!("Hello, world!");
    // string_up();
    format_test();
}

// String 被视为一个带有一些额外保证、限制和功能的字节 vector 的封装。

fn string_new() {
    let mut s = String::new();


    // 向 s 中 装载数据，通常字符串会有初始数据
    // 可以使用 to_string 方法，它能用于任何实现了 Display trait 的类型
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    // 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
    let hello = String::from("ﻢﻜﻴﻠﻋ مﻼﺴﻟا");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("םולש");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn string_up() {
    // 使用 push_str 方法来附加字符串 slice, 从而使 String 变长
    // push_str 方法参数采用字符串 slice, 因为我们并不需要获取参数的所有权。
    let mut s = String::from("foo");
    s.push_str("bar");
    // push 方法定义为获取一个单独的字符作为参数，并附加到 String 中。
    s.push('l');

    // 使用 + 运算符或 format! 宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 在相加后不再有效的原因，和使用 s2 的应用的原因，与使用 + 运算符时调用的函数签名有关。
    // + 运算符使用了 add 函数，它的签名是 fn add(self, s: &str) -> String
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // add 函数指定第二个参数是 &str, 在这个示例中 参数是 &String 类型，而不是 &str 类型。
    // 这是因为 &String 可以被 强转(coerced) 成 &str.
    // 当 add 函数被调用，Rust 使用了一个被称为 Deref 强制转换(deref coercion) 的技术，可以理解为 它把 &s2 变成了 &s2[...]
    // 会报错。 add 函数签名中 add 获取了 self 的所有权，因为 self 没有使用 &, 这意味着 在之前 + 实例中，s1 的所有权被移动到 add 调用中。之后不再有效。 
    // let s4 =s1.add(&s2);
}

// 复杂字符串链接 可以使用 format! 宏
fn format_test() {
       // 使用 format! 宏
       let s5 = String::from("tic");
       let s6 = String::from("tac");
       let s7 = String::from("toe");
       let s8 = format!("{s5}-{s6}-{s7}");
       println!("{s8}")
}

// 在 Rust 中，使用索引语法访问 String 的一部分，会出现一个错误。
// 错误内容：the type `str` cannot be indexed by `{integer}`
// 问题原因： Rust 字符串不支持索引，
fn supilt_test_err() {
    let s1 = String::from("hello");
    let h = s1[0];   // s1 报错：string indices are ranges of `usize`
}

// 有三种相关方式可以理解字符串：字节、标量值和字形簇（接近人们眼中 字母 的概念）。

