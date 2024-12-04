// 引用与借用

// 引用：像一个指针，因为它是一个地址，我们可以由此访问存储与该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值
// 借用：创建一个 引用 的行为
// 可变引用限制： 如果你有一个对该变量的可变引用，你就不能在创建对该变量的引用.
// 可变引用限制2：不能在拥有不可变引用的同时拥可变引用
// 引用作用域：一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
// 引用规则：
//      1. 在任意给定的时间，要么只能有一个可变引用，要么只能有多个不可变引用
//      2. 引用必须总是有效的
fn main() {
    // variable_give_and_back_test();
    // references_change_test_err();
    // mutable_references_change_test();
    // immutable_mutable_references_change_test_err();
    // immutable_mutable_references_change_test_err2();
    // immutable_mutable_references_change_test();
    dangling_references_test();
}

// 一个新的 calculate_length 示例，将 原来实现中 变量声明和函数返回值中的元组代码都去除掉
// fn variable_give_and_back_test() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);            // & 符号就是引用，它们允许你使用值但不获取所有权。 传递值用 &s1
//     println!("The length of '{s1}' is {len}")
// }

// fn calculate_length(s: &String) -> usize {              // & 符号就是引用，它们允许你使用值但不获取所有权。获取值用 &String。s 是 String 的引用
//     s.len()
// }                                                       // 这里，s 离开了作用域。但因为他并不拥有引用值得所有权，所以什么也没发生。

// 会报错。 变量默认是不可变的，引用也一样。（默认）不允许修改引用的值
// fn references_change_test_err() {
//     let s = String::from("hello");
//     change_err(&s);
// }
// fn change_err(some_string: &String) {
//     some_string.push_str(", world");        // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }

// 可变引用
//
// fn mutable_references_change_test() {
//     let mut s = String::from("hello");
//     mutable_change(&mut s);
//     println!("{}", s);
//     mutable_change(&mut s);
//     println!("{}", s);
// }
// fn mutable_change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

//  可变引用限制1： 如果你有一个对该变量的可变引用，你就不能在创建对该变量的引用. 防止同一时间对同一数据存在多个可变引用
// fn multi_mutable_references_change_test_err() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;    //cannot borrow `s` as mutable more than once at a time
//     println!("r1:{}, r2:{}", r1, r2);
// }
// fn multi_mutable_references_change_test() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     }                                               //r1 在这里离开作用域，之后就可以创建 s 的一个新的引用

//     let r2 = &mut s;
// }

// 可变引用限制2：不能在拥有不可变引用的同时拥可变引用
// fn immutable_mutable_references_change_test_err() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;                       //cannot borrow `s` as mutable because it is also borrowed as immutable

//     println!("r1:{}, r2:{}, r3:{}", r1, r2, r3);
// }

// 可变引用限制2：不能在拥有不可变引用的同时拥可变引用
// fn immutable_mutable_references_change_test_err2() {
//     let mut s = String::from("hello");
//     let r3 = &mut s;                      
//     let r1 = &s;                                // cannot borrow `s` as immutable because it is also borrowed as mutable
//     let r2 = &s;                                // cannot borrow `s` as immutable because it is also borrowed as mutable
//     println!("r1:{}, r2:{}, r3:{}", r1, r2, r3);
// }

// 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
// fn immutable_mutable_references_change_test() {
//     let mut s = String::from("hello");
//     let r1 = &s;                            //没问题
//     let r2 = &s;                            //没问题
//     println!("r1:{}, r2:{}", r1, r2);                //此位置为 r1, r2 最后使用的位置，之后不再使用
//     let r3 = &mut s;                    //没问题
//     println!("r3:{}", r3);
// }

// 悬垂指针：其指向的内存已经被分配给其他持有者。
// Rust 不会发生 悬垂指针，当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域
// fn dangling_references_test_err() {
//     let references_to_nothing = dangle();
// }
// 当 dangle 代码执行完毕之后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String
// fn dangle() -> &String {                            // dangle 返回一个字符串引用
//     let s = String::from("hello");          // s 是 一个新的字符串
//     &s                                              // 返回字符串 s 的引用。 this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// }                                                   // 这里 s 离开作用域并被丢弃，其内存被释放。 危险！

fn dangling_references_test() {
    let references_to_nothing = dangle();
    println!("{references_to_nothing}");
}
// 当 dangle 代码执行完毕之后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String
fn dangle() -> String {                            // dangle 返回一个字符串引用
    let s = String::from("hello");          // s 是 一个新的字符串
    s                                              // 返回字符串 s,这就没有问题
}   