// 什么是所有权?

// 所有权 规则
// 1. Rues 中每个值又有一个所有者(Owner)。
// 2. 值在任意时刻有且只有一个所有者。
// 3.  当所有者（值）离开作用域时，这个值将被抛弃。
fn main() {
    // scope_test();
    // string_test();
    // variable_int_move();
    // variable_string_move();
    // variable_string_clone();
    // variable_function_test();
    // variable_owner_function_test();
    variable_give_and_back_test();
}

// ***变量的所有权总遵循相同的模式：将值赋值给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动到另一个变量所有。***

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
// fn variable_string_clone() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1={}, s2={}", s1, s2);
// }

// 将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或者复制，就像赋值语句一样
// fn variable_owner_function_test() {
//     let s = String::from("hello");      //s 进入作用域
//     takes_ownership(s);               //s 值移动到函数里....
//                                                 // ... 所以到这里 s 不再有效

//     let x = 5;                              // x 进入作用域
//     makes_copy(x);                   // x 应该移动函数里
//                                                 // 但是 i32 是 copy 的
//                                                 // 所以在后面可继续使用 x
// }
// fn takes_ownership(some_string: String) {       // some_string 进入作用域
//     println!("{}", some_string);
// }                                               // 这里，some_string 移出作用域并调用 `drop` 方法。
//                                                 // 占用内存释放

// fn makes_copy(some_integer: i32) {              // some_integer 进入作用域
//     println!("{}", some_integer);
// }                                               // 这里，some_integer 移出作用域。没有特殊之处


// 返回值也可以转移所有权
// fn variable_owner_function_test() {
//     let s1 = gives_ownership();                     // gives_owership 将返回值转移给 s1
//     let s2 = String::from("hello");                 // s2 进入作用域
//     let s3 = takes_and_gives_back(s2);      // s2 被移动到 takes_and_gives_back 中，它将返回值移动给 s3

//     println!("s1:{}, s3:{}", s1, s3)
// }                                                           // 这里，s3 移出作用域被丢弃。s2 也移出作用域，但已被移走，这里不会做什么。s1 离开作用域并被丢弃

// fn gives_ownership() -> String {                            // gives_ownership 会将返回值移动给调用它的函数
//     let some_string = String::from("yours");        // some_string 进入作用域
//     some_string                                             // 返回 some_string 并移出给调用的函数
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// } 


//想要函数使用变量，在函数结束后还要继续使用变量，并且想要函数产生的一些数据
fn variable_give_and_back_test() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}




