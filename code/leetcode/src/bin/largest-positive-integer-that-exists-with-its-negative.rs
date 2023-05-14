struct Solution;
fn main() {
    Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]);
}
impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_by_key(|v| v.abs());
        while nums.len() > 1 {
            let a = nums[nums.len() - 1];
            let b = nums[nums.len() - 2];
            if a + b == 0 {
                return a.abs();
            }
            nums.pop();
        }
        -1
    }
}
