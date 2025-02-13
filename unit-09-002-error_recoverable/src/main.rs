/* 用 Result 处理可恢复错误 */
use std::fs::File;
use std::io::{self, ErrorKind, Read};
// Result 枚举，定义了两个成员， OK 和 Err
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// T 和 E 是泛型类型参数，T 代表成功时返回的 OK 成员中的数据的类型，E 代表失败时返回 Err 成员中的错误的类型。
fn main() {
    // greeting_file_result_error();
    // greeting_file_result_or_create();
    // greeting_file_unwrap();
    // greeting_file_expect();
    // read_username_from_file().expect("Failed to read username from file");
    // read_username_from_file_short().expect("Failed to read username from file");
    last_char_of_first_line_test();
}

fn greeting_file_result_error () {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file:{error:?}"),
    };
}

/* 匹配不同的错误 */
fn greeting_file_result_or_create() {

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{error:?}"),
            },
            other_error => panic!("Problem opening the file:{other_error:?}"),
        }
    };
}

/* 失败时 panic 的简写： unwrap 和 expect */ 
// unwrap 是 Result<T, E> 类型定义的很多辅助方法之一。
// 实现类似 match 功能，如果 Result 值是成员Ok,unwrap 会返回 Ok 中的值，
// 如果 Result 是成员 Err, unwrap 会给我们调用 panic! 宏。
fn greeting_file_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}

// expect 和 unwrap 类似，但是可以自定义 panic! 的错误信息
fn greeting_file_expect() {
    let greeting_file = File::open("hello.txt")
    .expect("Failed to open hello.txt");
}

/* 传播错误 */
// 传播错误：当编写一个其实先回调用一些可能会失败的操作的函数时，
// 除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理
// 使用 Result<String, io::Error> 是因为 File::open 和 read_to_string 两个可能会失败的操作的错误返回值。
fn read_username_from_file() -> Result<String, io::Error> {

    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error), // 使用 return 提前结束整个函数，并将来自 File::open 的错误值作为函数错误值传回给调用者
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* 传播错误的简写: ? 运算符 */
fn read_username_from_file_short() -> Result<String, io::Error> {
    // Result 值后的 ? 被定义为与match表达式有着完全相同的工作方式。
    // 如果 Result 的值是 Ok, ? 运算符会将返回 Ok 中的值而程序继续执行。
    // 如果值是 Err, Err 将为作为整个函数的返回值，就好像使用了return 关键字一样，这样错误值就被传播给了调用者。
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 在 ? 运算符后直接使用链式方法调用，进一步缩短代码
fn read_username_from_file_short2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/* 哪里可以使用 ? 运算符 */
// ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。
// 因为 ? 运算符被定义为从函数中提早返回一个值。
// 1. 只能在返回 Result 后者其他实现了 FromResidual 的类型的函数中使用 ? 运算符。
// 2. 可用于 Option<T> 值。与在 Result<T, E> 上调用 ? 运算符类似：
//    如果值是 None, 此时 None 会从函数中提前返回。如果值是 Some, Some 中的值作为表达式的返回值同时函数继续
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn last_char_of_first_line_test() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );
    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

// 可以在返回 Result 的函数中对 Result 使用 ? 运算符，可以在返回 Option 的函数中对 Option 使用 ? 运算符，但不可混合搭配。
// 


