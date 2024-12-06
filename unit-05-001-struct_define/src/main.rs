// 结构体的定义和实例化

// struct,或者 structure, 是一个自定义数据类型，允许你包装和命名多个相关的值，从而形成有意义的值
// 结构体于元组类似，他们都包含多个相关的值。
// 和元组一样，结构体每个部分都可以是不同类型。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
// 结构体相对于元组，不需要依赖顺序指定或访问示例中的值
fn main() {
    // init_user();

    // mut_init_user();
    // hide_init_user();
    // struct_update_old();
    // struct_update_new();
    // tuple_struct();
}

//定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。接着，在打括号中，定义每一部分数据的名字和类型，我们称为 字段(field)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// // 创建一个实例，需要以结构体名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段，其中 key 是字段名，value 是需要存储在字段中的数据值。
// // 实例中 字段的顺序不需要和他们在结构体中的声明顺序一致。
// fn init_user() {
//     let user1 = User {
//         active: true,
//         username: String::from("some_user"),
//         email: String::from("some_user@example.com"),
//         sign_in_count: 1
//     };
//     println!("user info -> active:{}, username:{}, email:{}, sign_in_count:{}", user1.active, user1.username, user1.email, user1.sign_in_count);
// }

// 如果结构体实例是可变的，可以使用点号为对应字段赋值
// ** 注意： 整个实例必须是可变的。Rust 不允许只将某个字段标记为可变。
// fn mut_init_user() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("some_user"),
//         email: String::from("some_user@example.com"),
//         sign_in_count: 1 
//     };

//     user1.email = String::from("mut@example.com");
//     println!("user info -> active:{}, username:{}, email:{}, sign_in_count:{}", user1.active, user1.username, user1.email, user1.sign_in_count);
// }

// 可以在函数体的最后一个表达式中构造一个结构体实例，来隐式的返回这个实例
// fn hide_init_user() {
//     let user1 = build_user(
//         String::from("some_user@example.com"), 
//         String::from("some_user")
//     );
//     println!("user info -> active:{}, username:{}, email:{}, sign_in_count:{}", user1.active, user1.username, user1.email, user1.sign_in_count);
// }

// common 函数隐式返回实例
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1
//     }
// }

//field init shorthand：参数名与字段名都完全相同，我们可以使用字段初始化简写语法(field init shorthand)
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,                                                   // 参数名与字段名相同，简写语法
//         email,                                                      // 参数名与字段名相同，简写语法
//         sign_in_count: 1
//     }
// }

// //结构体部分字段更新, 不适用结构更新语法，创建 user2 实例，为 email 字段设置新的值
// fn struct_update_old() {
//     let user1 = User {
//         active: true,
//         username: String::from("some_user") , 
//         email: String::from("some_user@example.com"),
//         sign_in_count: 1
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username, 
//         email: String::from("user2@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     println!("user2 info -> active:{}, username:{}, email:{}, sign_in_count:{}", user2.active, user2.username, user2.email, user2.sign_in_count);
// }

// //结构体更新语法（struct update syntax）： 使用 .. 指定了剩余未显示设置值的字段应有与给定实例对应字段相同的值
// // ..user1 必须放在最后，以指定其余的字段应从 user1 的相应字段中获取其值，但我们可以以任意顺序为任意字段指定值，而不用考虑结构体定义中字段顺序
// // struct update syntax 就像带有 = 的赋值，因为它移动了数据。 在创建 user2 之后就不能使用 user1 了。
// // 但是如果我们 我们对 user2 的 email 和 username 都赋了新 String 值, 从而只使用 user1 的 active 和 sign_in_count 的，那么 user1 在 user2 创建后仍然有效。
// //  这是因为 active 和 sign_in_count 的类型是实现了 Copy trait 的类型。 
// fn struct_update_new() {
//     let user1 = User {
//         active: true,
//         username: String::from("some_user") , 
//         email: String::from("some_user@example.com"),
//         sign_in_count: 1
//     };

//     let user2 = User { 
//         email: String::from("user2@example.com"),
//         ..user1
//     };

//     println!("user2 info -> active:{}, username:{}, email:{}, sign_in_count:{}", user2.active, user2.username, user2.email, user2.sign_in_count);
//     // 这里会报错  value borrowed here after move。move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
//     // println!("user1 info -> active:{}, username:{}, email:{}, sign_in_count:{}", user1.active, user1.username, user1.email, user1.sign_in_count);

//     // 这个就不会报错。因为 user2 中的 email 是新赋值的 String, 没有移动 user1 中的 email 赋值给 user2。active 和 sign_in_count 的类型是实现了 Copy trait 的类型。
//     println!("user1 info -> active:{},  email:{}, sign_in_count:{}", user1.active, user1.email, user1.sign_in_count);
// }

// 元组结构体(tuple struct): 有着结构体名称提供的含义，但是没有具体的字段名，只有字段的类型。
// 定义 元组结构体：以 struct 关键字和结构体名称开头，并 后跟元组中类型
// #[derive(Debug)]                                                        // Debug 元素个数小于12 的元组打印
// struct Color(i32, i32, i32);
// #[derive(Debug)]
// struct Point(i32, i32, i32);

// // black 和 origin 值的类型不同，因为他们是不同的元组结构体实例。
// fn tuple_struct() {
//     let black = Color(0, 1, 2);
//     println!("black:{:?}", black);
//     let origin = Point(3, 4, 5);
//     println!("origin:{:?}", origin);

//     let black2 = Color(black.0, origin.2, black.2);
//     println!("black2:{:?}", black2);
// }

// 类单元结构体(unit-like structs)：没有任何字段的结构体
// unit-like structs 常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据时发挥作用
struct AlwaysEqual;
fn unit_like_struct() {
    let subjetc = AlwaysEqual;
}
