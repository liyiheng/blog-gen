struct Solution;
fn main() {
    dbg!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]) == 18);
    dbg!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]) == 12);
    dbg!(Solution::max_sum_div_three(vec![4]) == 0);
    dbg!(Solution::max_sum_div_three(vec![2, 19, 6, 16, 5, 10, 7, 4, 11, 6]) == 84);
    dbg!(Solution::max_sum_div_three(vec![2, 6, 2, 2, 7]) == 15);
    dbg!(Solution::max_sum_div_three(vec![2, 3, 36, 8, 32, 38, 3, 30, 13, 40]) == 195);
}
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        // dp[0][i]:  nums[..i] 能被 3 整除的最大和
        // dp[1][i]:  nums[..i] 除以 3 余 1 的最大和
        // dp[2][i]:  nums[..i] 除以 3 余 2 的最大和
        let mut dp = vec![vec![i32::MIN; nums.len()]; 3];
        let remaind = nums[0] as usize % 3;
        dp[remaind][0] = nums[0];
        for (i, n) in nums.into_iter().enumerate().skip(1) {
            let remaind = n as usize % 3;
            match remaind {
                0 => {
                    dp[0][i] = 0.max(dp[0][i - 1]) + n;
                    dp[1][i] = dp[1][i - 1] + n;
                    dp[2][i] = dp[2][i - 1] + n;
                }
                1 => {
                    dp[0][i] = (dp[2][i - 1] + n).max(dp[0][i - 1]);
                    dp[1][i] = (dp[0][i - 1] + n).max(dp[1][i - 1]).max(n);
                    dp[2][i] = (dp[1][i - 1] + n).max(dp[2][i - 1]);
                }
                2 => {
                    dp[0][i] = (dp[1][i - 1] + n).max(dp[0][i - 1]);
                    dp[1][i] = (dp[2][i - 1] + n).max(dp[1][i - 1]);
                    dp[2][i] = (dp[0][i - 1] + n).max(dp[2][i - 1]).max(n);
                }
                _ => unreachable!(),
            }
        }
        dp[0].last().cloned().map(|v| v.max(0)).unwrap()
    }
}
