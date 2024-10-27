fn main() {
    println!("++++++++++++ Start celsius_fahrenheit_transf ++++++++++++");
    let to_f: bool = true;
    let temperature: f64 = 103.0;
    let result_temp = celsius_fahrenheit_transf(temperature, to_f);
    if to_f {
        println!("{}°C -> {}°F", temperature, result_temp);
    } else {
        println!("{}°F -> {}°C", temperature, result_temp);
    }
    println!("++++++++++++ End celsius_fahrenheit_transf ++++++++++++");

    println!("++++++++++++ Start fibonacci_number ++++++++++++");
    for fibonacci_number in 1..4 {
        let return_fibonacci_value: u128 = gen_fibonacci_number(fibonacci_number);
        println!("第{fibonacci_number}个斐波那契数是{return_fibonacci_value}");
    }
    println!("++++++++++++ End fibonacci_number ++++++++++++");

    println!("++++++++++++ Start print_song_of_christmas ++++++++++++");
    print_song_of_christmas();
    println!("++++++++++++ End print_song_of_christmas ++++++++++++");
    
}

// 相互转换摄氏与华氏温度。
// 摄氏度（C）= 5×（华氏度（F）- 32）/9
// 华氏度（F）= 9×摄氏度（C）/5+32
// temperature: 温度
// toF: true[转成华氏度]，false[转成摄氏度]
fn celsius_fahrenheit_transf(temperature: f64, to_f: bool) -> f64 {
    if to_f {
        9.0 * temperature / 5.0 + 32.0
    } else {
        5.0 * (temperature - 32.0) / 9.0
    }
}

// 生成第 n 个斐波那契数。
fn gen_fibonacci_number(number: u32) -> u128 {
    if number == 1 {
        1
    } else if number == 2 {
        1
    } else {
        gen_fibonacci_number(number-1) + gen_fibonacci_number(number-2)
    }
}

// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
fn print_song_of_christmas() {
    //定义 天数 元数据
    let day_str = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", 
        "ninth", "tenth", "eleventh", "twelfth"];
    let send_things = ["A partridge in a pear tree", "Two turtle doves and", "Three French hens,", 
            "Four calling birds,", "Five golden rings.", "Six geese a-laying,", "Seven swans a-swimming,", 
            "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", 
            "Twelve drummers drumming,"];
    


    for day_number in 0..11 {
        
        print!("On the {} day of Christmas, my true love sent to me", day_str[day_number]);
        for day_count in (0..day_number).rev() {
            print!(" {}", send_things[day_count]);
        }
        println!(".");
    }


}


