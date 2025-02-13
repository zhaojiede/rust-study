use rand::Rng;
use crate::after_test::test1_median_mode;
use crate::after_test::test2_pig_latin;
use crate::after_test::test3_company;



pub mod after_test;
fn main() {
    mddian_mode_test();
    // conversion_to_pig_latin_test();
}

fn conversion_to_pig_latin_test() {
    let pig_latin_vec = test2_pig_latin::conversion_to_pig_latin("old");
    println!("pig_latin_vec: {}", pig_latin_vec);
}

fn mddian_mode_test() {
    let mut random_vec = random_vec(10);
    let (median, mode_vec) = test1_median_mode::median_mode(&mut random_vec);

    let mode_vec_str =mode_vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
    println!("median: {}, mode_vec: {}", median, mode_vec_str);
    
}

fn random_vec(length: usize) -> Vec<i32> {
    // 定义随机数生成器
    let mut rng = rand::thread_rng();
    // 生成随机 Vec<i32>
    (0..length).map(|_| rng.gen_range(0..100)).collect()
}

