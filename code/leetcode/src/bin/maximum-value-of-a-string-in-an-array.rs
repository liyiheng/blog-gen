struct Solution;
fn main() {
    _ = Solution::maximum_value;
}
fn parse(dat: &[u8]) -> i32 {
    let mut v = 0;
    for &b in dat.iter() {
        if b.is_ascii_digit() {
            v *= 10;
            v += (b - b'0') as i32;
        } else {
            return dat.len() as i32;
        }
    }
    v
}
impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter().map(|s| parse(s.as_bytes())).max().unwrap()
    }
}
