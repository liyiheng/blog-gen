struct Solution;
fn main() {
    Solution::largest_odd_number("".to_owned());
}

impl Solution {
    pub fn largest_odd_number(mut num: String) -> String {
        while !num.is_empty() {
            let v = *num.as_bytes().last().unwrap() - b'0';
            if v % 2 == 0 {
                num.pop();
            } else {
                break;
            }
        }
        num
    }
}
