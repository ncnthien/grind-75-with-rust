use std::collections::HashMap;
struct Solution;

impl Solution {
    fn two_sum(nums: Vec<usize>, target: usize) -> Vec<usize> {
        let mut recording_list: HashMap<usize, usize> = HashMap::new();
        let mut index: usize = 0;
        let mut result: Vec<usize> = vec![];

        while index < nums.len() {
            let num = nums[index];
            if let Some(&first_index) = recording_list.get(&(target - num)) {
                result.push(first_index);
                result.push(index);
                break
            } else {
                recording_list.insert(num, index);
            }
            index += 1;
        }
        result
    }
}

fn main() {
    let nums: Vec<usize> = vec![2, 7, 11, 15];
    let target = 9;
    
    println!("the result is {:?}", Solution::two_sum(nums, target));
}
