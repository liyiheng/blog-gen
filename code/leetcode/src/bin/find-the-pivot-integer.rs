struct Solution;
fn main() {
    dbg!(Solution::pivot_integer(8));
    dbg!(Solution::pivot_integer(4));
}
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut a = 1;
        let mut b = n;
        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            if a < b {
                lo += 1;
                a += lo;
            } else {
                hi -= 1;
                b += hi;
            }
        }
        if a == b {
            lo
        } else {
            -1
        }
    }
}
