impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let a = num % 9;
        if a == 0 {
            9
        } else {
            a
        }
    }
}

struct Solution;
fn main() {}
