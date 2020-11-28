#![allow(dead_code)]
fn main() {}
struct Solution;
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        use std::collections::HashMap;
        let mut cache = HashMap::new();

        let mut result = vec![];
        for (&l, &r) in l.iter().zip(r.iter()) {
            if r - l < 2 {
                result.push(true);
                continue;
            }
            if let Some(&f) = cache.get(&(l, r)) {
                result.push(f);
                continue;
            }
            let sub = &nums[l as usize..=r as usize];
            let mut v = vec![0; sub.len()];
            v.clone_from_slice(sub);
            v.sort();
            let d = v[0] - v[1];
            result.push(true);
            cache.insert((l, r), true);
            for i in 2..v.len() {
                if d != v[i - 1] - v[i] {
                    cache.insert((l, r), false);
                    result.pop();
                    result.push(false);
                    break;
                }
            }
        }
        result
    }
}
