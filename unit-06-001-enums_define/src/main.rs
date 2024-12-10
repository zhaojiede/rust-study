// 枚举的定义

// 枚举给予你一个途径去声明某个值是一个集合中的一员。
fn main() {
    println!("Hello, world!");
}

// // 定义一个枚举：列出可能的 IP 地址类型
// // 枚举测成员位于其标识符的命名空间内，并使用两个冒号分开。
// enum IpAddrKind {
//     V4,
//     V6,
// } 

// fn enum_define_test() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(IpAddrKind::V4);                //枚举测成员位于其标识符的命名空间内，并使用两个冒号分开。  
//     route(IpAddrKind::V6);
// }
// fn route(ip_kind: IpAddrKind) {}  

// // 直接将将数据附加到枚举的每个成员上，。
// // 每一个我们定义的枚举成员的名称也变成了一个构建枚举实例的函数
// // IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn enum_define_test2() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// // 枚举每个成员可以处理不同类型和数量的数据。
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn enum_define_test3() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// Option<T> : 一个可以编码存在或不存在概念的枚举
// Option<T> 不需要显示引用作用域。它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None
fn option_define_test() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}





