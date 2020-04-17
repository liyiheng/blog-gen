use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        Solution::step(&candidates, target)
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect()
    }

    pub fn step(candidates: &[i32], target: i32) -> HashSet<Vec<i32>> {
        let mut all = HashSet::new();
        for (i, &c) in candidates.iter().take_while(|c| **c <= target).enumerate() {
            if c == target {
                all.insert(vec![c]);
                continue;
            }
            let partial = Solution::step(&candidates[i + 1..], target - c);
            for v in partial.into_iter() {
                let mut v = v.clone();
                v.push(c);
                all.insert(v);
            }
        }
        all
    }
}

struct Solution {}

fn main() {
    let r = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    println!("{:?}", r);
    let r = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    println!("{:?}", r);
}
