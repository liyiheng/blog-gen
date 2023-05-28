struct Solution;
fn main() {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total = 0;
        let (mut a, mut b) = (0, 0);
        for t in time_series {
            if b > a {
                total += b - a;
            }
            a = t.max(b);
            b = t + duration;
        }
        if b > a {
            total += b - a;
        }
        total
    }
}
