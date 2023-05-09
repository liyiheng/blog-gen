struct Solution;

pub fn main() {}
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut remaind = 1 % k;
        set.insert(remaind);
        let mut ans = 1;
        while remaind != 0 {
            ans += 1;
            remaind = (remaind * 10 + 1) % k;
            if set.contains(&remaind) {
                return -1;
            }
            set.insert(remaind);
        }
        ans
    }
}
