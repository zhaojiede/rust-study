// Slice 类型

// Slice 允许你引用集合中一段连续的元素序列，而不是引用整个集合。
// Slice 是一个引用，所以没有所有权。
// 字符串字面值就是一个 Slice。 【let s = "Hello, world!";】 这里的 s 类型是 &str: 它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的。 
fn main() {
    // first_word_no_slice();
    // string_slice_test();
    // first_word_test();
    first_word_test2();
}

// 获取字符串中第一个单词。
// 使用这种方法，索引与字符串是割裂的，需要随时保证三个飘忽不定的不相关的变量同步
// fn first_word_no_slice() {
//     let s = String::from("My name is xiaozhao.");
//     let index = first_word_index_no_slice(&s);
//     println!("first word index is {index}");
// }
// // 返回第一个单词结尾的索引
// fn first_word_index_no_slice(some_string: &String) -> usize {
//     let bytes = some_string.as_bytes();
    
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     some_string.len()
// }

// 字符串 Slice (String Slice): 是 String 中一部分值的引用
// &string_some[starting_index..ending_index]: 可以使用一个中括号中的[starting_index..ending_index]指定的 range 创建一个 slice, 
// 其中starting_index 是 slice 的第一个位置，ending_index 是 slice 的最后一个位置的后一个值
// Slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index-starting_index 的值
// fn  string_slice_test() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];       
//     let world = &s[6..11];
//     println!("{hello},{world}");

//     let hello2 = &s[..5];                       //对于 Rust 的 ..range 语法，如果想要从索引 0 开始，可以不写两个点之前的值
//     println!("no start_index:{hello2}");

//     let world2 = &s[6..];                       //依次类推，如果 slice 包含 String 最后一个字节，也可以舍弃尾部的数字
//     println!("no end_index:{world2}");

//     let all = &s[..];                           //也可以同时舍弃这两个值获取这个字符串的 slice
//     println!("no start_index and end_index:{all}");

//     let all = &s;                               // ****保留一个疑问 &s 于 &s[..] 有什么不一样？****
//     println!("no start_index and end_index:{all}");
// }

// 重写 获取第一个单词 测试
// fn first_word_test() {
//     // let s = String::from("My name is xiaozhao.");
//     let s = String::from("Mynameisxiaozhao.");
//     let first_word1 = first_word(&s);
//     println!("The first word of 【{s}】 is {first_word1}");

// }
// fn first_word(s: &String) -> &str {                     // “字符串 slice” 的类型声明写作 &str
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..]                                              // 经测试 &s[..] 于 &s 都能正常返回整个字符串
// }

// 重写 获取第一个单词 测试。适配 String 类型参数，同时也适配 String 的 Slice 作为参数 
fn first_word_test2() {
    // let s = String::from("My name is xiaozhao.");
    let s = String::from("My name is xiaozhao.");

    // 适用于 `String` 的引用，这等价于整个 `String` 的 Slice
    let first_word1 = first_word2(&s);
    println!("The first word1 of 【{s}】 is {first_word1}");

    let first_word2 = first_word2(&s[0..6]);
    println!("The first word2 of 【{s}】 is {first_word2}");

    let my_string = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let first_word3 = first_word2(&my_string[0..6]);
    // println!("The first word3 of 【{my_string}】 is {first_word3}");

    let first_word4 = first_word2(&my_string[..]);
    // println!("The first word4 of 【{my_string}】 is {first_word4}");

    let first_word5 = first_word2(my_string);
    // println!("The first word5 of 【{my_string}】 is {first_word5}");

    let my_string = String::from("hello world");
    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    println!("The first word5 of 【{my_string}】 is {word}");
    let word = first_word(&my_string[..]);
    println!("The first word5 of 【{my_string}】 is {word}");
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);
    println!("The first word5 of 【{my_string}】 is {word}");
    let my_string_literal = "hello world";
    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    println!("The first word5 of 【{my_string_literal}】 is {word}");
    let word = first_word(&my_string_literal[..]);
    println!("The first word5 of 【{my_string_literal}】 is {word}");
    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
    println!("The first word5 of 【{my_string_literal}】 is {word}");

}
fn first_word(s: &str) -> &str {                     // “字符串 slice” 的类型声明写作 &str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]                                              // 经测试 &s[..] 于 &s 都能正常返回整个字符串
}

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }


// fn main() {
//     let my_string = String::from("hello world");
//     // `first_word` 适用于 `String`（的 slice），部分或全部
//     let word = first_word(&my_string[0..6]);
//     println!("The first word5 of 【{my_string}】 is {word}");
//     let word = first_word(&my_string[..]);
//     println!("The first word5 of 【{my_string}】 is {word}");
//     // `first_word` 也适用于 `String` 的引用，
//     // 这等价于整个 `String` 的 slice
//     let word = first_word(&my_string);
//     println!("The first word5 of 【{my_string}】 is {word}");
//     let my_string_literal = "hello world";
//     // `first_word` 适用于字符串字面值，部分或全部
//     let word = first_word(&my_string_literal[0..6]);
//     println!("The first word5 of 【{my_string_literal}】 is {word}");
//     let word = first_word(&my_string_literal[..]);
//     println!("The first word5 of 【{my_string_literal}】 is {word}");
//     // 因为字符串字面值已经 **是** 字符串 slice 了，
//     // 这也是适用的，无需 slice 语法！
//     let word = first_word(my_string_literal);
//     println!("The first word5 of 【{my_string_literal}】 is {word}");
// }

