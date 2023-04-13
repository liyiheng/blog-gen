struct Solution;
pub fn main() {
    Solution::max_profit(vec![]);
}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut prev = 0;
        for (i, &p) in prices.iter().enumerate() {
            if i != 0 {
                if p > prev {
                    ans += p - prev;
                }
            }
            prev = p;
        }
        ans
    }
}
