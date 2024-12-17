/* if let 简单控制流 */

// if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。
fn main() {
    un_if_let_test();
}

fn un_if_let_test() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

// if let 语法获取通过等号分割的一个模式和一个表达式。
// 工作模式与 match 相同，表达式对应 match 而模式对应第一个分支。
// 模式：Some(max)
// 模式不匹配时 if let 中代码不会执行 
fn if_let_test() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
// 在 if let 中包含一个 else。else 块中代码与 match 表达式的 _ 分支块中代码相同。
fn if_let_else_test() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        ()
    }
}