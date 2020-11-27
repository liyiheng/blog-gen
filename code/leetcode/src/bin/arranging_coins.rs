#![allow(dead_code)]
fn main() {}
struct Solution;
impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        let mut width = 0;
        loop {
            if n <= width {
                return width;
            }

            width += 1;
            n -= width;
        }
    }
}
