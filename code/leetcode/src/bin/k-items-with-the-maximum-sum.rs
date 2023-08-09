struct Solution;
fn main() {
    dbg!(Solution::k_items_with_maximum_sum(0, 0, 0, 0));
}
impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        mut k: i32,
    ) -> i32 {
        let mut ans = 0;
        if k >= num_ones {
            ans += num_ones;
            k -= num_ones;
        } else {
            return k;
        }
        if k >= num_zeros {
            k -= num_zeros;
        } else {
            return ans;
        }
        ans - k
    }
}
