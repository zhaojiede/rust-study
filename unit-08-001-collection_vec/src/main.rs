/* 常见集合 */
// Vec<T> 也被称为vector。vector 允许我们一个挨一个地存储一系列数量可变的值
// 字符串(string) 是字符的集合。
// 哈希 map (hash map) 允许我们将值与一个特定的键（key) 相关联，

// Vec<T>,也被称为 vector。vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻排列所有值。
// vector 只能储存相同类型的值。
// Rust 在编译时必须确切知道 vector 中的类型，这样它才能确定在堆上需要为每个元素分配多少内存。

fn main() {
    println!("Hello, world!");
}

// 我们增加了一个注解类型。因为没有向这个 vector 中插入任何值，Rust 并不知道我们要存储什么类型的元素。
// vector 是用泛型实现的。
fn create_vec() {
    let v: Vec<i32> = Vec::new();
}

// 通常，我们会用初始值来创建一个 Vec<T> 而 Rust 会推断出储存值的类型
fn create_vec_macro() {
    let v = vec![1, 2, 3];
}

// 使用 push 方法增加元素。
fn update_vec() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
}

// 两种方式读取 vector 中的元素：
// 1. 通过索引。 索引是从数字 0 开始的。使用 & 和 [] 语法来获取一个元素的引用。
// 2. 通过 get 方法。当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>。
fn read_vec() {
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

// 会报错：不能在相同作用域中同时存在可变和不可变引用的规则。
//    代码：获取 vector 第一个元素的不可变引用 并 尝试在 vector 末尾增加一个元素。
// 原因：vector的工作方式。在 vector 的结尾增加新元素时，在没有足够空间将所有元素意思相邻存放的情况下，
//    Rust 可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种情况。
fn borrow_vec_err() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];          // 不可变引用
    v.push(6);
    println!("first: {first}");
}

// 遍历：可以无需通过索引一次一次的访问。
fn ergodic_vec() {
    let v = vec![100, 32,57];
    for i in &v {
        println!("{}", i);
    }
}

// 可变遍历：可以遍历可变 vector 的每一个元素的可变引用以便能改变它们。
fn ergodic_vec_mut() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
