struct Solution;
fn main() {
    // 0  0,1,2,3,4
    // 1  0,1,2
    // 2  0
    dbg!(Solution::ways_to_buy_pens_pencils(20, 10, 5));
}
impl Solution {
    pub fn ways_to_buy_pens_pencils(mut total: i32, mut cost1: i32, mut cost2: i32) -> i64 {
        if cost1 < cost2 {
            std::mem::swap(&mut cost1, &mut cost2);
        }
        let mut ans = 0;
        while total >= 0 {
            let pencil_ways = 1 + total / cost2;
            ans += pencil_ways as i64;
            total -= cost1
        }
        ans
    }
}
