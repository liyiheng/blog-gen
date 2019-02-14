impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut third = std::i32::MIN;
        let mut stack = vec![];
        for n in nums.into_iter().rev() {
            if n < third {
                return true;
            }
            while stack.len() > 0 && *stack.last().unwrap() < n {
                third = stack.pop().unwrap();
            }
            stack.push(n);
        }
        return false;
    }
}

struct Solution;

fn main() {
    assert_eq!(false, Solution::find132pattern(vec![]));
    assert_eq!(false, Solution::find132pattern(vec![1, 2, 3, 4]));
    assert_eq!(true, Solution::find132pattern(vec![1, 3, 2]));
    assert_eq!(true, Solution::find132pattern(vec![3, 1, 4, 2]));
    assert_eq!(true, Solution::find132pattern(vec![-1, 3, 2, 0]));
    assert_eq!(true, Solution::find132pattern(vec![3, 5, 0, 3, 4]));
}
