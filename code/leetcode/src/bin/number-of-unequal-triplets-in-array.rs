struct Solution;
fn main() {}
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let total = nums.len();
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *counter.entry(n).or_default() += 1;
        }
        let counts: Vec<i32> = counter.into_values().collect();
        let mut ans = 0;
        let mut prev_cnt = 0;
        let mut after_cnt = total as i32;
        for c in counts {
            ans += prev_cnt * c * (after_cnt - c);
            after_cnt -= c;
            prev_cnt += c;
        }
        ans
    }
}
