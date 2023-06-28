struct Solution;
fn main() {}
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        use std::collections::HashMap;
        let mut counter: HashMap<String, i32> = HashMap::new();
        nums.into_iter().for_each(|s| {
            *counter.entry(s).or_default() += 1;
        });
        let mut ans = 0;
        for i in 1..target.len() {
            let (first, second) = target.split_at(i);
            let first_n = counter.get(first).cloned().unwrap_or_default();
            if first == second {
                ans += first_n * (first_n - 1);
                continue;
            }
            let second_n = counter.get(second).cloned().unwrap_or_default();
            ans += first_n * second_n;
        }
        ans
    }
}
