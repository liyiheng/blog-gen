struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        if n == 0 || x == 1.0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == 2 {
            return x * x;
        }
        if x == -1.0 {
            if n % 2 == 0 {
                return 1.0;
            }
            return -1.0;
        }
        if n < 0 {
            let neg_n = n.checked_mul(-1);
            if neg_n.is_none() {
                return 0.0;
            }
            let v = Solution::my_pow(x, neg_n.unwrap());
            if v.is_infinite() {
                return 0.0;
            }
            return 1.0 / v;
        }
        if n % 2 == 1 {
            return Solution::my_pow(Solution::my_pow(x, n / 2), 2) * x;
        } else {
            return Solution::my_pow(Solution::my_pow(x, n / 2), 2);
        }
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.0, 10));
    println!("{}", Solution::my_pow(2.1, 3));
    println!("{}", Solution::my_pow(2.00000, -2147483648));
}
