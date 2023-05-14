#![allow(dead_code)]
fn main() {
    println!("{}", Solution::min_time_to_type("abc".to_string()));
    println!("{}", Solution::min_time_to_type("bza".to_string()));
    println!("{}", Solution::min_time_to_type("zjpc".to_string()));
}
struct Solution;
// https://leetcode-cn.com/contest/biweekly-contest-59/problems/minimum-time-to-type-word-using-special-typewriter/
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut seconds = 0;
        let mut cur_pos = b'a';
        for &b in word.as_bytes() {
            let offset = if b > cur_pos {
                b - cur_pos
            } else {
                cur_pos - b
            };
            let step = if offset > 13 {
                26 - offset as i32
            } else {
                offset as i32
            };
            seconds += step + 1;
            cur_pos = b;
        }
        seconds
    }
}
