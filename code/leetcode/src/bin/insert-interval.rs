struct Solution;
fn main() {
    Solution::insert(vec![], vec![]);
}
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let i = match intervals.binary_search_by_key(&new_interval[0], |v| v[0]) {
            Ok(i) => i,
            Err(i) => i,
        };
        intervals.insert(i, new_interval);
        let mut ans = vec![];
        for cur_span in intervals {
            if ans.is_empty() {
                ans.push(cur_span);
                continue;
            }
            let prev: &Vec<i32> = ans.last().unwrap();
            if prev[1] < cur_span[0] {
                // [prev]-[cur]
                ans.push(cur_span);
                continue;
            }
            if prev[1] > cur_span[1] {
                // --[  prev  ]---
                // -----[cur]-----
                continue;
            }
            if prev[1] >= cur_span[0] {
                // [prev]
                // ---[cur]
                let prev = ans.last_mut().unwrap();
                prev[1] = cur_span[1];
            }
        }
        ans
    }
}
