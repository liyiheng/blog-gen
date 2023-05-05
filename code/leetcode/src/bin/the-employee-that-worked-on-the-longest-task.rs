struct Solution;
fn main() {
    let logs = [[0, 3], [2, 5], [0, 9], [1, 15]]
        .map(|v| v.to_vec())
        .to_vec();
    Solution::hardest_worker(10, logs);
}
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut m = 0;
        for i in 0..logs.len() {
            let dur = logs[i][1] - if i == 0 { 0 } else { logs[i - 1][1] };
            if dur > m {
                m = dur;
            }
        }
        let mut ans = n;
        for i in 0..logs.len() {
            let dur = logs[i][1] - if i == 0 { 0 } else { logs[i - 1][1] };
            if dur < m {
                continue;
            }
            let id = logs[i][0];
            ans = ans.min(id);
        }
        ans
    }
}
