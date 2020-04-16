impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        use std::collections::HashSet;
        let mut all = HashSet::new();
        Solution::step(vec![], &candidates, target, &mut all);
        all.into_iter().collect()
    }
    pub fn step(
        cur: Vec<i32>,
        candidates: &[i32],
        target: i32,
        all: &mut std::collections::HashSet<Vec<i32>>,
    ) {
        if target == 0 {
            return;
        }
        for i in candidates.iter() {
            if *i > target {
                break;
            }
            let mut c = cur.clone();
            c.push(*i);
            if *i == target {
                c.sort();
                all.insert(c);
            } else {
                Solution::step(c, candidates, target - i, all);
            }
        }
    }
}

struct Solution {}

fn main() {
    let r = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    println!("{:?}", r);
    let r = Solution::combination_sum(vec![2, 3, 5], 8);
    println!("{:?}", r);
}
