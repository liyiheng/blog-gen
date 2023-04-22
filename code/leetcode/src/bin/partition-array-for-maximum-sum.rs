fn main() {
    Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3);
}
struct Solution;
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![-1; arr.len()];
        dp[0] = arr[0];
        for i in 0..arr.len() {
            if i < k {
                let m = *arr[..=i].iter().max().unwrap();
                let s = m * (i + 1) as i32;
                dp[i] = dp[i].max(s);
                continue;
            }
            // [j, i)
            for j in i - k as usize + 1..=i {
                let m = *arr[j..=i].iter().max().unwrap();
                let s = m * (i - j + 1) as i32;
                dp[i] = dp[i].max(dp[j - 1] + s);
            }
        }
        *dp.last().unwrap()
    }
}
