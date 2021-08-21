#![allow(dead_code)]

struct Solution;
fn main() {}

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let dat = unsafe { s.as_bytes_mut() };
        let k = k as usize;
        let mut start = 0;
        while start + 1 < dat.len() {
            let end = if start + k > dat.len() {
                dat.len()
            } else {
                start + k
            };
            dat[start..end].reverse();
            start += k * 2;
        }
        s
    }
}
