struct Solution;
fn main() {
    dbg!(Solution::largest_vals_from_labels(
        vec![5, 4, 3, 2, 1],
        vec![1, 1, 2, 2, 3],
        3,
        1
    ));
}
impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        use std::collections::HashMap;
        // (value, label)
        let mut set: Vec<(i32, i32)> = values.into_iter().zip(labels.into_iter()).collect();
        set.sort_by_key(|v| v.0);
        let mut ans = 0;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut cnt = 0;
        for (v, l) in set.iter().rev() {
            if cnt == num_wanted {
                break;
            }
            let lc = counter.get(l).cloned().unwrap_or_default();
            if lc < use_limit {
                ans += *v;
                *counter.entry(*l).or_default() += 1;
                cnt += 1;
            }
        }
        ans
    }
}
