#![allow(dead_code)]
fn main() {}
struct Solution;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (x, y) = ops.into_iter().fold(
            (m, n),
            |(x, y), p| (x.min(p[0]), y.min(p[1])),
        );
        x * y
    }
}
