struct Solution;
fn main() {
    let a = vec![13, 5, 1, 8, 21, 2];
    Solution::maximum_tastiness(a, 3);
}
impl Solution {
    fn check(price: &[i32], k: i32, delta: i32) -> bool {
        let mut n = 1;
        let mut prev = price[0];
        for &i in &price[1..] {
            if i - delta >= prev {
                n += 1;
                prev = i;
            }
        }
        n >= k
    }
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort();
        let mut hi = price[price.len() - 1] - price[0];
        let mut lo = 0;
        // 二分
        // 甜度 mid 的礼盒是否存在
        // 若存在，检查更甜的是否存在
        // 否则降低甜度再次检查
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if Solution::check(&price, k, mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }
}
