struct Solution;
fn main() {}
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .map(|s| {
                let a = s.as_bytes()[11];
                let b = s.as_bytes()[12];
                (a - b'0') * 10 + (b - b'0')
            })
            .filter(|&v| v > 60)
            .count() as i32
    }
}
