impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut y: i32 = 0;
        while x != 0 {
            let a = y.checked_mul(10);
            if a.is_none() {
                return 0;
            }
            let b = a.unwrap().checked_add(x % 10);

            if b.is_none() {
                return 0;
            }
            y = b.unwrap();
            x /= 10;
        }
        y
    }
}

struct Solution {}

fn main() {
    assert_eq!(12345, Solution::reverse(54321));
    assert_eq!(0, Solution::reverse(1023456789));
    assert_eq!(-12345, Solution::reverse(-54321));
}
