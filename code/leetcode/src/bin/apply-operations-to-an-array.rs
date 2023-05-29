#![allow(dead_code)]
struct Solution;
fn main() {}
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut non_zero_idx = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums[non_zero_idx] = nums[i];
                non_zero_idx += 1;
            }
        }
        let last = *nums.last().unwrap();
        if last != 0 {
            nums[non_zero_idx] = last;
            non_zero_idx += 1;
        }
        nums[non_zero_idx..].iter_mut().for_each(|n| *n = 0);
        nums
    }
}
