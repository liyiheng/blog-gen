impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits.len() == 0 {
            return vec![1];
        }
        let mut extra = 0;
        let last = digits.last_mut().unwrap();
        *last += 1;
        let mut i = digits.len();
        while i > 0 {
            i -= 1;
            digits[i] += extra;
            extra = 0;
            if digits[i] >= 10 {
                digits[i] -= 10;
                extra = 1;
            }
        }
        if extra > 0 {
            digits.insert(0, 1);
        }
        digits
    }
}

struct Solution;

fn main() {
    Solution::plus_one(vec![]);
}
