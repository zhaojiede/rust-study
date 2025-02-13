/* 使用 Hash Map 储存键值对 */

/* HashMap<K, V> 类型存储了一个键类型 K 和一个值类型 V 的映射。
    它通过一个哈希函数来实现映射，决定如何将键和值放入内存中。
    像 vector 一样，哈希 map 将它们的数据储存在堆上。
*/
use std::collections::HashMap;

fn main() {
    // new_hash_map();
    // hash_map_ower_test();
    hash_map_update_test();
}

// 类似于 vector, 哈希 map 是同质的：所有键必须是相同类型，值也必须都是相同类型。
fn new_hash_map() {
    // 通过 new 函数创建一个空的 HashMap
    let mut scores = HashMap::new();
    // 使用 insert增加元素
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow:"), 50);

    // 通过 get 方法并提供对应的键来从 HashMap 中获取值
    let team_name = String::from("Blue");
    // get 方法返回 Option<&V>, 如果某个键在 HashMap 中没有对应的值， get 会返回 None.
    // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>, 
    // 接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // 遍历 HashMap
    for (key, value) in &scores  {
        println!("{key}:{value}")
    }
}

// 对于像 i32 这样的实现 Copy trait 的类型，其值可以拷贝进 HashMap.
// 对于像 String 这样拥有所有权的值，其值将被移动而 HashMap 会成为这些值的所有制
// 如果将值的引用插入 HashMap, 这些值本身将不会被移动到 HashMap. 
// 但这些引用指向的值必须至少在 HashMap 有效时也是有效的
fn hash_map_ower_test() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 这里 field_name 和 field_value 不再有效，
    // 使用会有编译报错【value borrowed here after move】
    // println!("{field_name}:{field_value}");
}

fn hash_map_update_test() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 如果插入一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    
    // entry 函数获取我们想要检查的键作为参数。
    // entry 返回值是一个枚举，Entry, 他代表的可能存在也可能不存在的值。
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // {"Blue": 25, "Yellow": 50}
    println!("{scores:?}");
    
    let text = "hello world wondeful world";
    let mut map = HashMap::new();
    // split_whitespace 方法返回一个由空格分隔 text 值 子slice的迭代器。
    // or_insert 方法返回这个减的值的一个可变引用(&mut V)。
    // 这里我们将可变引用储存在 count变量中，所以为了赋值必须使用星号(*)解引用。
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // {"world": 2, "hello": 1, "wondeful": 1}
    println!("{map:?}");

}

