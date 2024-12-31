/* 将模块拆分成多个文件 */
// ** 注意 **：你只需要在模块树中的某处使用一次 mod 声明就可以加载这个文件。

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}