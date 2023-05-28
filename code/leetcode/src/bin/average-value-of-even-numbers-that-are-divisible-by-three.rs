struct Solution;
fn main() {}
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let nums: Vec<i32> = nums.into_iter().filter(|v| v % 6 == 0).collect();
        if nums.is_empty() {
            0
        } else {
            nums.iter().sum::<i32>() / nums.len() as i32
        }
    }
}
