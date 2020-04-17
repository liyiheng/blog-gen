impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut all = Solution::step(&candidates, target);
        for v in all.iter_mut() {
            v.sort();
        }
        all
    }
    pub fn step(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut all = vec![];
        for (i, &c) in candidates.iter().take_while(|c| **c <= target).enumerate() {
            if c == target {
                all.push(vec![c]);
                continue;
            }
            let partial = Solution::step(&candidates[i..], target - c);
            for v in partial.into_iter() {
                let mut v = v.clone();
                v.push(c);
                all.push(v);
            }
        }
        all
    }
}

struct Solution {}

fn main() {
    let r = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    println!("{:?}", r);
    let r = Solution::combination_sum(vec![2, 3, 5], 8);
    println!("{:?}", r);
}
