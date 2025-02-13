/*
将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加“ay”，所以 “first” 会变成 “irst-fay”。
元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
牢记 UTF-8 编码！
*/
pub fn conversion_to_pig_latin(word: &str) -> String {
    let mut result = String::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if is_vowel(first_char) {
        result.push_str(&word);
        result.push('-');
        result.push_str("hay");
    } else {
        result.push_str(&chars.collect::<String>());
        result.push('-');
        result.push(first_char);
        result.push_str("ay");
    }
    result
        
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}