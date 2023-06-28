struct Solution;
fn main() {
    dbg!(Solution::find_non_min_or_max(vec![]));
}
impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &n in nums.iter() {
            min = min.min(n);
            max = max.max(n);
        }
        nums.into_iter()
            .find(|&n| n != min && n != max)
            .unwrap_or(-1)
    }
}
