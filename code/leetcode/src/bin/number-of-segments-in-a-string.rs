struct Solution;
fn main() {
    Solution::count_segments("".to_owned());
}
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .count() as i32
    }
}
