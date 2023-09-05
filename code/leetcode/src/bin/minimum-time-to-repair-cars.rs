struct Solution;
fn main() {
    dbg!(Solution::repair_cars(vec![4, 2, 3, 1], 10));
}
use std::collections::HashMap;
impl Solution {
    fn calc(ranks: &[i32], minutes: i64, mem: &mut HashMap<i64, i64>) -> i64 {
        let mf = minutes as f64;
        match mem.get(&minutes) {
            None => {
                let n = ranks
                    .iter()
                    .map(|&v| v as f64)
                    .map(|v| mf / v)
                    .map(|v| v.sqrt())
                    .map(|f| f as i64)
                    .sum();
                mem.insert(minutes, n);
                n
            }
            Some(n) => *n,
        }
    }
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut mem = HashMap::<i64, i64>::new();
        let cars = cars as i64;
        let boss = *ranks.iter().min().unwrap() as i64;
        let max_minutes = boss * cars * cars;
        let mut lo = 0;
        let mut hi = max_minutes;
        while lo <= hi {
            let mid = (hi - lo) / 2 + lo;
            let n = Solution::calc(&ranks, mid, &mut mem);
            let prev_n = Solution::calc(&ranks, mid - 1, &mut mem);
            if n >= cars && prev_n < cars {
                return mid;
            }
            if n < cars {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        0
    }
}
