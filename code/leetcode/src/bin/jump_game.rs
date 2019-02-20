impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut max = 0;
        let mut i = 0;
        while i <= max && i < l as i32 {
            let a = i + nums[i as usize];
            if a > max {
                max = a;
            }
            i += 1;
        }
        i == l as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(true, Solution::can_jump(vec![2, 5, 0, 0]));
    assert_eq!(true, Solution::can_jump(vec![2, 0]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
