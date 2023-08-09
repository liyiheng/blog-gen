struct Solution;
fn main() {
    dbg!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3));
    dbg!(Solution::number_of_subarrays(vec![2, 4, 6], 1));
    dbg!(Solution::number_of_subarrays(
        vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2],
        2
    ));
}
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        counter.insert(0, 1);
        let mut n = 0;
        for &v in nums.iter() {
            n += v % 2;
            *counter.entry(n).or_default() += 1;
        }
        let zero_num = counter.get(&0).cloned().unwrap_or_default();
        let mut ans = counter.get(&k).cloned().unwrap_or_default() * zero_num;
        for x in (k + 1)..=n {
            let a = counter.get(&x).cloned().unwrap_or_default();
            let b = counter.get(&(x - k)).cloned().unwrap_or_default();
            ans += a * b;
        }
        ans
    }
}
