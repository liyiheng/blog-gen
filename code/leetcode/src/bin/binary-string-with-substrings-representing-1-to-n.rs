struct Solution;
fn main() {
    println!("{:b}", 7);
}
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for i in 1..=n {
            let x = format!("{:b}", i);
            if !s.contains(&x) {
                return false;
            }
        }
        true
    }
}
