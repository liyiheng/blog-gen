struct Solution;
pub fn main() {
    Solution::count_days_together(
        "10-01".to_string(),
        "10-31".to_string(),
        "10-13".to_string(),
        "12-21".to_string(),
    );
}

const DAYS: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
fn to_days(v: &str) -> i32 {
    let (m, d) = v.split_once("-").unwrap();
    let mut day = 0;
    for i in 0..m.parse().unwrap() {
        day += DAYS[i];
    }
    day + d.parse::<i32>().unwrap()
}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let aa = to_days(&arrive_alice);
        let la = to_days(&leave_alice);
        let ab = to_days(&arrive_bob);
        let lb = to_days(&leave_bob);
        (la.min(lb) - aa.max(ab) + 1).max(0)
    }
}
