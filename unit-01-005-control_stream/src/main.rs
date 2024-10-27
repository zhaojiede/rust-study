fn main() {
    println!("++++++++++++ Start if_test ++++++++++++++");
    if_test(9);
    println!("++++++++++++ Start let_if_test ++++++++++++++");
    let_if_test();
    println!("++++++++++++ Start loop_test ++++++++++++++");
    loop_test();
    println!("++++++++++++ Start loop_break_test ++++++++++++++");
    loop_break_test();
    println!("++++++++++++ Start while_test ++++++++++++++");
    while_test(7);
    println!("++++++++++++ Start for_test ++++++++++++++");
    for_test();
    println!("++++++++++++ Start for_rand_test ++++++++++++++");
    for_rand_test();
    
}

// 条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误:expected `bool`, found `i32`
fn if_test (number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
fn let_if_test() {
    let condition: bool = true;

    // if 的每个分支的可能的返回值都必须是相同类型.否则，编译期有报错：expected integer, found `&str`
    // let number = if condition {5} else {"six"};
    let number = if condition {5} else {6};
    println!("The value of number is:{number}");
}

fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn loop_break_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_test(mut number: i32) {
    while number != 0 {
        println!("The value of number is: {number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }
}

fn for_rand_test() {
    // 代码不会踏足到数字 4，仅从一个数字开始到另一个数字之前。
    for number in (1..4).rev() {
        println!("The value of number is: {number}");
    }
    println!("LIFTOFF!!!");
}
