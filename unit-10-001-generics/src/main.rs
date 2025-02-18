/* 泛型、Trait 和生命周期 */
// 泛型(generics): 具体类型和其他属性的抽象代替。

/* 泛型数据类型 */
fn main() {
    let number_list = vec![1,4,6,9,20,54];
    let result = largest_err(&number_list);
    println!("The largest number is {}", result);
}

// 在函数定义中使用泛型
// 为了参数化新函数中的类型，需要为类型参数命名，道理和给函数的形参起名一样。任何标识符都可以作为类型参数的名字。
// 传统上来说，Rust 的类型参数名字都比较短，通常仅为一个字母，同时，Rust 类型名的命名规范是首字母大写驼峰式命名方法。
// T 作为 “Type” 的缩写是大部分 Rust 程序员的首选。
// 为了在函数签名中使用一个类型参数时，必须在使用它之前就声明它。
// 类型参数声明位于函数名称与参数列表中间的加括号<>中。 fn largest<T>(list:&[T]) -> &T {}

// 会报错 binary operation `>` cannot be applied to type `&T`, 表明 largest 的函数体不能适用于 T 的所有可能类型。
// fn largest_err<T>(list: &[T]) -> &T {
// 加上 <T: std::cmp::PartialOrd>  不会报错，我们限制 T 只对实现了 std::cmp::PartialOrd 的类型有效，就可以编译了
fn largest_err<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 结构体定义中的泛型
// 可以用<> 语法来定义结构体，它包含一个或多个泛型参数类型字段。
// 必须在结构体名后面的<>中声明泛型参数名称。接着可以在结构体定义中可以指定具体数据类型位置使用泛型类型
// Point<T> 结构体定义了一个 Point 结构体，它有一个泛型参数 T。Point 结构体有两个字段 x 和 y，它们都是泛型类型 T。
struct Point<T> {
    x: T,
    y: T,
}

// 想要定义一个 x 和 y 可以有不同类型且仍然是泛型的 Point 结构体，可以使用多个泛型类型参数。
struct Point2<T, U> {
    x: T,
    y: U,
}

// 方法定义中使用泛型
// 在位结构体和枚举实现方法时，一样也可以使用泛型。
// 注意：必须在 impl 后面声明 T, 这样就可以在 Point<T> 上实现的方法中使用T了
// 我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同的名称，不过依照惯例使用了相同的名称。

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 定义方法时也可以为泛型指定限制。例如，可以选择为 Point<f32> 实例实现方法，而不是为了泛型 Point 实例
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
// 这个示例为了展示一些泛型国通 impl 声明而另一些通过方法定义声明声明泛型的情况
fn mixup_test() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 枚举定义中的泛型
enum Resulted<T, E> {
    Ok(T),
    Err(E),
}