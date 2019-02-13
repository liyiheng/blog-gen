impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut prev = std::i32::MIN;
        for i in 0..nums.len() {
            let n = nums[i];
            if target > prev && target <= n {
                return i as i32;
            }
            prev = n;
        }
        nums.len() as i32
    }
}

struct Solution;

fn main() {}
