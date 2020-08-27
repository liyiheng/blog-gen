impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<Option<i32>> = vec![None; nums.len()];
        for i in 0..nums.len() {
            let m = dp[i].unwrap_or_default();
            let v = nums[i];
            for j in i + 1..=i + v as usize {
                if j == nums.len() {
                    break;
                }
                let n = dp[j].unwrap_or(2100000000);
                dp[j] = Some(n.min(m + 1));
            }
        }
        dp.pop().unwrap_or_default().unwrap_or_default()
    }
}

struct Solution;

fn main() {
    assert_eq!(0, Solution::jump(vec![0]));
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    assert_eq!(3, Solution::jump(vec![1, 2, 3, 0, 1, 4]));
    assert_eq!(
        5,
        Solution::jump(vec![
            5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4, 1,
            6, 8, 8, 4, 4, 2, 0, 3, 8, 5
        ])
    );
    assert_eq!(10000, Solution::jump(vec![1; 10001]));
}
