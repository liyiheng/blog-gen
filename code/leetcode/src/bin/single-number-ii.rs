struct Solution;
fn main() {
    let a = vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2];
    dbg!(Solution::single_number(a));
}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *counter.entry(n).or_default() += 1;
        }
        for (k, v) in counter.into_iter() {
            if v == 1 {
                return k;
            }
        }
        return -1;
    }
}
