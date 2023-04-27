struct Solution;
fn main() {
    Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 2, 3);
}
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut maxes = vec![];
        let mut max1 = 0;
        let mut max2 = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        for (i, &v) in nums.iter().enumerate() {
            sum1 += v;
            if i + 1 > first_len {
                sum1 -= nums[i - first_len];
            }
            if i + 1 >= first_len {
                max1 = max1.max(sum1);
            }
            sum2 += v;
            if i + 1 > second_len {
                sum2 -= nums[i - second_len];
            }
            if i + 1 >= second_len {
                max2 = max2.max(sum2);
            }
            maxes.push((max1, max2));
        }
        let mut maxes_rev = vec![];
        let mut max1 = 0;
        let mut max2 = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        for (i, &v) in nums.iter().enumerate().rev() {
            sum1 += v;
            if i + first_len < nums.len() {
                sum1 -= nums[i + first_len];
            }
            if i + first_len <= nums.len() {
                max1 = max1.max(sum1);
            }
            sum2 += v;
            if i + second_len < nums.len() {
                sum2 -= nums[i + second_len];
            }
            if i + second_len <= nums.len() {
                max2 = max2.max(sum2);
            }
            maxes_rev.push((max1, max2));
        }
        maxes_rev.reverse();
        let mut ans = 0;
        for i in 1..nums.len() {
            let a = maxes[i - 1].0 + maxes_rev[i].1;
            let b = maxes[i - 1].1 + maxes_rev[i].0;
            ans = ans.max(a).max(b);
        }
        println!("nums {:?}", nums);
        println!("max {:?}", maxes);
        println!("maxrev {:?}", maxes_rev);
        ans
    }
}
