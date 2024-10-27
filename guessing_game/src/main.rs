use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 打印提示
    println!("Guess the number!");

    //生成一个随机数
    let secred_number = rand::thread_rng().gen_range(1..=100);
    println!("The secred number is:{secred_number}");

    
    loop {
        //提示用户输入数字
        println!("Please input your guess:");
        let mut guess = String::new();
        // 获取用户输入
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        // 将用户输入值转换为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };

        // 打印用户获取数值
        println!("You guessed:{guess}");

        // 用户输入与生成数字比较
        match guess.cmp(&secred_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        }
    }
    

   
}
 