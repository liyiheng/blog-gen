struct Solution;
pub fn main() {
    Solution::multiply(1, 2);
}
use std::collections::HashMap;
impl Solution {
    pub fn multiply(a: i32, b: i32) -> i32 {
        let mut mem = HashMap::new();
        if a.abs() <= b.abs() {
            Solution::multiply_x(a, b, &mut mem)
        } else {
            Solution::multiply_x(b, a, &mut mem)
        }
    }
    fn multiply_x(a: i32, b: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        if a < 0 {
            return -Solution::multiply_x(-a, b, mem);
        }
        if let Some(&v) = mem.get(&a) {
            return v;
        }
        if a < 3 {
            let mut ans = 0;
            for _ in 0..a {
                ans += b;
            }
            mem.insert(a, ans);
            return ans;
        }
        let x = a / 2;
        let y = a - x;
        let ans = Solution::multiply_x(x, b, mem) + Solution::multiply_x(y, b, mem);
        mem.insert(a, ans);
        ans
    }
}
