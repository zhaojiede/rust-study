// Slice 类型

// Slice 允许你引用集合中一段连续的元素序列，而不是引用整个集合。
// Slice 是一个引用，所以没有所有权。
// 字符串字面值就是一个 Slice。 【let s = "Hello, world!";】 这里的 s 类型是 &str: 它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的。 
fn main() {
    // first_word_no_slice();
    // string_slice_test();
    // first_word_test();
    // first_word_test2();
    slice_arrary();
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
// 其中starting_index 是 slice 的第一个位置，ending_index 是 slice 的最后一个位置+1
// Slice 的【数据结构】存储了 slice 的开始位置和长度，长度对应于 ending_index-starting_index 的值
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
// fn first_word_test2() {
//     // let s = String::from("My name is xiaozhao.");
//     let s = String::from("My name is xiaozhao.");

//     // 适用于 `String` 的引用，这等价于整个 `String` 的 Slice
//     let first_word1 = find_first_word(&s);
//     println!("The first word1 of 【{s}】 is {first_word1}");

//     let first_word2 = find_first_word(&s[0..6]);
//     println!("The first word2 of 【{s}】 is {first_word2}");

//     let my_string = "hello world";

//     // `first_word` 适用于字符串字面值，部分或全部
//     let first_word3 = find_first_word(&my_string[0..6]);
//     println!("The first word3 of 【{my_string}】 is {first_word3}");

//     let first_word4 = find_first_word(&my_string[..]);
//     println!("The first word4 of 【{my_string}】 is {first_word4}");

//     // 因为字符串字面值已经是字符串 Slice,这也是使用的，无需 Slice 
//     let first_word5 = find_first_word(my_string);
//     println!("The first word5 of 【{my_string}】 is {first_word5}");

// }
// fn find_first_word(s: &str) -> &str {                     // “字符串 slice” 的类型声明写作 &str
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..]                                              // 经测试 &s[..] 于 &s 都能正常返回整个字符串
// }

fn slice_arrary() {
    let a = [1, 2, 3, 4, 5];

    // slice start_index:开始值的 index; end_indes:结束值的index+1
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}


