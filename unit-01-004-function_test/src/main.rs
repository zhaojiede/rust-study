fn main() {
    //没有参数，没有返回值函数，函数变量名遵循 snake case 规范风格
    // another_function();

    //带参数函数
    print_labeled_measurement(6, 'h');

    let plus_v = plus_one(8);
    println!("8 + 1 等于：{plus_v}");
}

// fn another_function() {
//     println!("Another function.");
// }

// 带参数函数，函数签名中，必须声明每个参数的类型
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is:{value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


