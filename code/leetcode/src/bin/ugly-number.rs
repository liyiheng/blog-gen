struct Solution;
fn main() {
    Solution::is_ugly(123);
}
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let shrink = |mut v, m| -> i32 {
            while v % m == 0 {
                v /= m;
            }
            v
        };
        n = shrink(n, 5);
        n = shrink(n, 3);
        n = shrink(n, 2);
        n == 1
    }
}
