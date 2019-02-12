impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        let negtive = (dividend < 0) != (divisor < 0);
        if dividend < 0 {
            dividend = -dividend;
        }
        if divisor < 0 {
            divisor = -divisor;
        }
        let mut result: i64 = 0;
        while dividend >= divisor {
            let mut tmp = divisor;
            let mut part = 1;
            while dividend >> 1 >= tmp {
                tmp <<= 1;
                part <<= 1;
            }
            dividend -= tmp;
            result += part;
        }
        if negtive {
            result = 0 - result;
        }
        result as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(2147483647, Solution::divide(2147483647, 1));
    assert_eq!(2147483647, Solution::divide(-2147483648, -1));
    assert_eq!(-1073741824, Solution::divide(-2147483648, 2));
    assert_eq!(3, Solution::divide(10, 3));
    assert_eq!(-3, Solution::divide(7, -2));
}
