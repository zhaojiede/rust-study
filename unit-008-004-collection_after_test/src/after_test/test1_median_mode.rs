
use std::collections::HashMap;
/*  
给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）
和众数（出现次数最多的值；在这里哈希 map 会很有帮助） 
*/
pub fn median_mode(num_vec : &mut Vec<i32>) -> (i32, Vec<i32>) {
    println!("num_vec: {:?}", num_vec);
    let num_len =  num_vec.len();
    quick_sort(num_vec, 0, num_len - 1);
    println!("after sort num_vec: {:?}", num_vec);
    let mut median = num_vec.get(0).cloned().unwrap();
    if num_len % 2 == 0 {
        median = (num_vec.get(num_len/2 - 1).cloned().unwrap() + num_vec.get(num_len/2).cloned().unwrap())/2;
    } else {
        median = num_vec.get(num_len/2).cloned().unwrap();
    }

    // 计算众数
    let mut mode = Vec::new();
    let mut num_map = HashMap::new();
    for i in 0..num_len {
        let num = num_vec.get(i).cloned().unwrap();
        let count = num_map.get(&num).cloned().unwrap_or(0);
        let new_count = count + 1;
        num_map.insert(num, new_count);

        if mode.len() > 0 {
            let mode_num =mode.get(0).cloned().unwrap();
            if num != mode_num {
                let mode_num_count =num_map.get(&mode_num).cloned().unwrap_or(0);
                if new_count > mode_num_count {
                    mode.clear();
                    mode.push(num);
                } else if new_count == mode_num_count {
                    mode.push(num);
                }
            }
        } else {
            mode.push(num);
        }
    }
    (median, mode)
}

fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let pivot_index = partition(nums, left, right);
        if pivot_index == 0 || pivot_index == nums.len() - 1 {
            return;
        }
        quick_sort(nums, left, pivot_index - 1);
        quick_sort(nums, pivot_index + 1, right);
    }
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut index = left;
    for i in left..right {
        if nums[i] <= pivot {
            nums.swap(index, i);
            index += 1;
        }
    }
    nums.swap(index, right);
    index
}