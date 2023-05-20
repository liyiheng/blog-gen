struct Solution;
fn main() {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut cur = 1;
        for &n in nums.iter() {
            cur *= n;
            ans = ans.max(cur);
            if n == 0 {
                cur = 1;
            }
        }
        cur = 1;
        for &n in nums.iter().rev() {
            cur *= n;
            ans = ans.max(cur);
            if n == 0 {
                cur = 1;
            }
        }
        ans
    }
}
