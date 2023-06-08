struct Solution;
fn main() {
    Solution::equal_pairs(vec![]);
}
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = grid.len();
        let mut counter = HashMap::new();
        for line in grid.iter() {
            *counter.entry(line).or_insert(0) += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            let col: Vec<i32> = (0..n)
                .map(|j| unsafe { *grid.get_unchecked(j).get_unchecked(i) })
                .collect();
            ans += counter.get(&col).cloned().unwrap_or(0);
        }
        ans
    }
}
