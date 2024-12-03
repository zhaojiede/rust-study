// 引用与借用

// 引用：像一个指针，因为它是一个地址，我们可以由此访问存储与该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值
// 借用：创建一个 引用 的行为
fn main() {
    variable_give_and_back_test();
}

// 一个新的 calculate_length 示例，将 原来实现中 变量声明和函数返回值中的元组代码都去除掉
fn variable_give_and_back_test() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);            // & 符号就是引用，它们允许你使用值但不获取所有权。 传递值用 &s1
    println!("The length of '{s1}' is {len}")
}

fn calculate_length(s: &String) -> usize {              // & 符号就是引用，它们允许你使用值但不获取所有权。获取值用 &String。s 是 String 的引用
    s.len()
}                                                       // 这里，s 离开了作用域。但因为他并不拥有引用值得所有权，所以什么也没发生。