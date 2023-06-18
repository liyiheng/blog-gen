struct Solution;
fn main() {}
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cnt = 0;
        let mut ans = 0;
        for &i in flips.iter() {
            cnt += 1;
            max = max.max(i);
            if max == cnt {
                ans += 1;
            }
        }
        ans
    }
}
