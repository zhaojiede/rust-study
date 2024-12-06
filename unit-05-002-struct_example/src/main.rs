// 结构体示例程序

// 计算长方形面积，从单独变量开始 -> 重构 -> 使用结构体代替变量实现
fn main() {
    // mut_rectangles();
    // tuple_rectangles();
    struct_rectangles();
}

// 使用两个变量表示长方形的 长和宽。这两个参数时相关联的，但是程序没有体现出这点
// fn mut_rectangles() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         mut_area(width, height)
//     );
// }
// fn mut_area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组。元组帮我们增加一些结构性，只需传一个参数。但是元组没有给出元素名称，所以计算变得更费解了
// fn tuple_rectangles() {
//     let rect = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         tuple_area(rect)
//     );
// }
// fn tuple_area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 使用结构体
// 增加相关联，意义明确
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
// 我们希望借用结构体，而不是获取它的所有权，这样 struct_rectangles 函数就继续享有 rect 的所有权进而继续使用它。
fn struct_rectangles() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect)
    );

    // 使用 printle! 宏，并不能打印结构体实例，报错：error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rect is {}", rect);
    // 只在打印处增加 :?。 报错：error[E0277]: `Rectangle` doesn't implement `Debug`
    // println!("rect is {:?}", rect); 

    // 在结构体 Rectangle 上一行添加 #[derive(Debug)]，不会报错了
    // 打印 
    // rect is Rectangle { width: 30, height: 50 }
    println!("rect is {:?}", rect); 

    // 在结构体 Rectangle 上一行添加 #[derive(Debug)]，不会报错了
    // 打印 内容更丰富
    // rect is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    println!("rect is {:#?}", rect); 
}
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
