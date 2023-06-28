struct Solution;
fn main() {
    dbg!(Solution::diagonal_prime(vec![]));
}
fn is_prime(v: i32) -> bool {
    match v {
        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 => true,
        1 => false, // 艹
        _ => {
            let max = (v as f64).sqrt().floor() as i32;
            for i in 2..=max {
                if v % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let last = nums.len() - 1;
        let mut ans = 0;
        for (i, line) in nums.into_iter().enumerate() {
            let a = line[i];
            let b = line[last - i];
            if is_prime(a) {
                ans = ans.max(a);
            }
            if is_prime(b) {
                ans = ans.max(b);
            }
        }
        ans
    }
}
