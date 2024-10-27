fn main() {
    //--------- Start 标量(scalar)
    //--------- Start 整形: 
    // 有符号：i8, i16, i32, i64, i128, isize。有符号的变体可以储存包含从 -(2n - 1) 到 2n - 1 - 1 在内的数字，这里 n 是变体使用的位数
    // 无符号：u8, u16, u32, u64, u128, usize。无符号的变体可以储存从 0 到 2n - 1 的数字
    // isize 和 usize 类型依赖运行程序的计算机架构,64 位架构上它们是 64 位的，32 位架构上它们是 32 位的。
    let integer_i8: i8 = -45;
    println!("The values of integer is: {integer_i8}");
    let integer_i8: i8 = 45;
    println!("The values of integer is: {integer_i8}");
    // u8 定义为无符号数值，如果为 u8 类型数字赋值为复数，则会报错：cannot apply unary operator `-` to type `u8`
    // let integer_u8: u8 = -45;
    // println!("The values of integer is: {integer_u8}")
    let integer_u8: u8 = 45;
    println!("The values of integer is: {integer_u8}");
    //--------- End 整形

    
    //--------- Start 浮点：浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64。浮点型都是有符号的
    let float_64 = 2.0;
    println!("The values of float_64 is: {float_64}");
    let float_32: f32 = 3.0;
    println!("The values of float_32 is: {float_32}");
    //--------- End 浮点

    
    //--------- Start 布尔型:布尔类型有两个可能的值：true 和 false。
    let bool_v = true;
    println!("The values of bool_v is:{bool_v}");
    let bool_v: bool = false;
    println!("The values of bool_v is:{bool_v}");
    //--------- End 布尔型
    
    
    //--------- Start 字符类型
    let char_v = 'z';
    println!("The values of char_v is: {char_v}");
    let char_v: char = 'ℤ';
    println!("The values of char_v is: {char_v}");
    let char_v: char = '😻';
    println!("The values of char_v is: {char_v}");
    //--------- End 字符类型
    //--------- End 标量(scalar)

    
    //--------- Start 复合类型(Compound types)
    //--------- Start 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 直接使用元组打印报错：`(i32, f64, u8)` cannot be formatted with the default formatter
    // println!("The values of tup is:{tup}");
    // 可直接通过 变量+点号(.)+值的索引的方式获取值
    println!("The values of tup.0 is:{}", tup.0);
    println!("The values of tup.1 is:{}", tup.1);
    println!("The values of tup.2 is:{}", tup.2);
    // 超出边界，编译报错：unknown field
    // println!("The values of tup.3 is:{}", tup.3);

    // 解构
    let(x, y, z) = tup;
    println!("The values of x is: {x}");
    println!("The values of y is: {y}");
    println!("The values of z is: {z}");
    //--------- End 元组

    //--------- Start 数组. Rust 中数组每个元素的类型必须相同且长度固定
    let array_v = [1, 2, 3, 4, 5];
    println!("The values of array_v[4] is:{}", array_v[4]);

    //在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    let array_v2: [i32; 5] = [205, 28, 378, 4555, 545];
    println!("The values of array_v2[3] is:{}", array_v2[3]);

    //还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组
    let array_v3 = [3; 5];
    println!("The values of array_v3[4] is:{}", array_v3[4]);
    //--------- End 数组
    //--------- End 复合类型(Compound types)

}
