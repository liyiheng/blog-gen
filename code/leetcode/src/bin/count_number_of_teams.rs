#![allow(dead_code)]
struct Solution;
fn main() {}
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        if rating.len() < 3 {
            return 0;
        };
        let mut total = 0;
        for i_mid in 1..rating.len() - 1 {
            let i = rating[i_mid];
            let mut gt_a = 0;
            let mut gt_b = 0;
            let mut lt_a = 0;
            let mut lt_b = 0;
            let a = &rating[..i_mid];
            let b = &rating[i_mid + 1..];
            a.iter().for_each(|&e| {
                if e < i {
                    gt_a += 1;
                } else {
                    lt_a += 1;
                }
            });
            b.iter().for_each(|&e| {
                if e < i {
                    gt_b += 1;
                } else {
                    lt_b += 1;
                }
            });
            total += gt_a * lt_b;
            total += lt_a * gt_b;
        }
        total
    }
}
