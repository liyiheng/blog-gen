struct Solution;
fn main() {
    dbg!(Solution::number_of_cuts(8));
    dbg!(Solution::number_of_cuts(1));
    dbg!(Solution::number_of_cuts(2));
}
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        match n {
            1 => 0,
            _ => match n % 2 {
                0 => n / 2,
                _ => n,
            },
        }
    }
}
