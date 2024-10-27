// 常量的声明。声明常量后没有使用，在 build 时会有告警提示：constant `THREE_HOURS_IN_SECONDS` is never used
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    //---------- Start 变量 可变-不可变
    // // 定义变量，ruse 默认不可变
    // let x = 5;
    // println!("变量值为：{x}");
    // // 直接更改变量则报错: cannot assign twice to immutable variable
    // x = 6;
    // println!("变量值为：{x}");
    
    // // 使用 let mut 定义 可变变量
    // let mut x = 5;
    // println!("变量值为：{x}");
    // // 使用 mut 声明变量为可变变量
    // x = 6;
    // println!("变量值为：{x}");
    //---------- End 变量 可变-不可变

    // //---------- Start 常量 
    // //常量定义使用 const 生命，可以在任意位置定义，在作用域内都可以使用
    // println!("常量值：{THREE_HOURS_IN_SECONDS}");
    // //---------- End 常量 

    //---------- Start 隐藏
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The Value of x in the inner scope is: {x}");
    } 

    println!("The value of x is: {x}");

    // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
    let spaces = "    ";
    println!("The Value of spaces in the inner scope is: {spaces}");
    let spaces = spaces.len();
    println!("The Value of spaces in the inner scope is: {spaces}");
    //---------- End 隐藏

    

}
