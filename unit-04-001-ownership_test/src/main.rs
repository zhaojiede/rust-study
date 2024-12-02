
// 所有权 规则
// 1. Rues 中每个值又有一个所有者(Owner)。
// 2. 值在任意时刻有且只有一个所有者。
// 3.  当所有者（值）离开作用域时，这个值将被抛弃。
fn main() {
    // scope_test();
    // string_test();
    // variable_int_move();
    // variable_string_move();
    variable_string_clone();
}

// 作用域是一个项(item)在程序中有效的范围
// fn scope_test() {
//     let s = "hello";
//     println!("{}", s);
// }

// 除了字符串字面值，Rust 有另一个字符串类型，String。String 类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本。
// fn string_test() {
//     let mut s = String::from("hello");

//     s.push_str(", world");

//     println!("{}", s);
// }

// 将 5 绑定到 x；接着 生成一个值 x 的 拷贝并绑定到 y
// 因为整数是已知固定大小的简单值，所有两个 5 都被放入了栈中。
// fn variable_int_move() {
//     let x = 5;
//     let y = x;
//     println!("x={}, y={}", x, y);
// }

// String 在栈上存储内容，由三部分组成：
//      一个指向存放字符串内容内存的指针；
//      一个长度（String 的内容当前使用了多少字节的内存）；
//      一个容量（String 从分配器中总共获取了多少字节的内存）。
// String 内存存储在堆上
// Rust 永远也不会自动创建数据的 “深拷贝”。
// fn variable_string_move_err() {
//     let s1 = String::from("hello");

//     //将 s1 赋值给 s2，String 的数据被复制了，意味着 从栈上拷贝了它的 指针、长度、容量。没有复制指针指向的堆上数据。
//     //为解决 二次释放(double free)（当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。二次释放会导致内存污染） 的错误，
//     //  Rust 认为 在 let s2 = s1; 之后， s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。
//     let s2 = s1;
//     println!("s1={}, s2={}", s1, s2); // 报错 【value borrowed here after move】
// }

// 当需要深度复制 String 中 堆上 数据，而不仅仅是栈上数据时，可以使用一个叫做 clone 的函数
fn variable_string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={}, s2={}", s1, s2);
}


