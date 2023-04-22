struct Solution;
fn main() {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                left[i] = nums[i];
                continue;
            }
            left[i] = nums[i] * left[i - 1];
        }
        for i in (0..nums.len()).rev() {
            if i + 1 == nums.len() {
                right[i] = nums[i];
                continue;
            }
            right[i] = nums[i] * right[i + 1];
        }
        let mut ans = vec![];
        for i in 0..nums.len() {
            if i == 0 {
                ans.push(right[i + 1]);
            } else if i + 1 == nums.len() {
                ans.push(left[i - 1]);
            } else {
                ans.push(left[i - 1] * right[i + 1]);
            }
        }
        ans
    }
}
