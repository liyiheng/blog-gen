struct Solution;
fn main() {}
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            n * 2
        }
    }
}
