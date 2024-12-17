// match 控制流结构

// match: 极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
// 模式可由字面值、变量、通配符和许多其他内容构成。(第 十八 章介绍不同类型模式)
// 值会通过 match 的每一个模式，并在遇到第一个符合的模式时，值会进入相关联的代码块并在执行中被使用。
fn main() {
    // values_in_cents_test();
    // value_bind_cents_test();
    // value_plus_onetest();
    wildcard_test();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 开起来非常象 if 所使用的条件表达式，但区别非常大：对于 if, 表达式必须返回一个 bool 值，但是 match 可以返回任意类型
// 一个 分支 由两部分：一个模式和一些代码。第一个分支的模式是 Coin::Penny 而之后的 => 运算符将模式和要运行的代码分开。
// 每个分支用 逗号 分开
// 当 match 表达式执行时，他将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。
//      如果模式并不匹配值，将继续执行下一个分支。
// 可以拥有任意多的分支
// 每个分支相关联的代码是一个表达式，而表达式的结果将作为整个 match 表达式的返回值。
// 如果想要在分支中运行多行代码，可以使用大括号，分治后的 逗号是可选的
fn values_in_cents(conin: Coin) -> u8 {
    match conin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn values_in_cents_test() {
    let penny = Coin::Penny;
    let val = values_in_cents(penny);
    println!("penny val:{}", val);
}


/* 绑定值模式 */
#[derive(Debug)]
enum UsState {
    Alabam,
    Alska,
}
enum CionBind {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_bind_cents(coin: CionBind) -> u8 {
    match coin {
        CionBind::Penny => 1,
        CionBind::Nickel => 5,
        CionBind::Dime => 10,
        CionBind::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn value_bind_cents_test() {
    let alska = CionBind::Quarter(UsState::Alska);
    value_bind_cents(alska);
}

/* 匹配 Option<T> */
fn value_plus_onetest() {
    let six = value_plus_one(Some(5));
    let seven = value_plus_one(Some(6));
    let none = value_plus_one(None);
}

fn value_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(5) => {                            // value_plus_one(Some(5)) 示例中 Some(5) 与 Some(5) 匹配
            println!("five match 5");
            Some(6)
        },
        Some(i) => {                        // value_plus_one(Some(6)) 示例中 Some(i) 与 Some(6) 匹配，他们是相同的成员
            println!("five match i");
            Some(i +1)
        }                       
    }
}

/* 匹配时穷尽的 */
// 分支必须覆盖所有可能。Rust 中的匹配时穷尽的(exhaustive):必须穷举到最后的可能性来使代码有效。
// value_plus_one_err 编译会报错: pattern `None` not covered
// fn value_plus_one_err(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(5) => {                            
//             println!("five match 5");
//             Some(6)
//         },
//         Some(i) => {                        
//             println!("five match i");
//             Some(i +1)
//         }                       
//     }
// }

/* 通配模式和 _ 占位符 */

// 通配符
// 最后一个模式匹配所有未被特殊列出的值。必须将通配分支放在最后。
fn wildcard_test() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),                     
        other => move_player2(other),                //涵盖所有其他可能值，模式是我们命名为 other 的一个变量
        tt => move_player1(tt),                      // 编译告警 no value can reach this 不会运行到这个方法, 但是能运行
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player1(num_spaces: u8) {
        println!("1 remove num_spaces:{num_spaces}")
    }
    fn move_player2(num_spaces: u8) {
        println!("2 remove num_spaces:{num_spaces}")
    }
}


// 下划线模式
// 当我们不想使用通配符中的变量值的时候，可以使用 _ 。这告诉 Rust 我们不适用这个值，所以Rust 也不会警告我们存在未使用的变量
fn underscores_test() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),                     
        _ => reroll(),

    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {
        println!("再来一局");
    }
}

// 特殊情况以外情况不做任何事情
// 明确告诉 Rust 我们不会使用与前面模式不匹配的值，并且这种情况我们不想运行任何代码
fn underscores_undo_test() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),                     
        _ => (),

    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}