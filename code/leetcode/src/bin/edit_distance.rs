struct Solution;
fn main() {
    Solution::min_distance("".to_string(), "hello".to_string());
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        for i in 0..=word1.len() {
            dp[i][0] = i;
        }
        for j in 0..=word2.len() {
            dp[0][j] = j;
        }
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j - 1]).min(dp[i - 1][j]);
                }
            }
        }
        dp.last()
            .map(|v| v.last())
            .flatten()
            .cloned()
            .unwrap_or_default() as i32
    }
}
