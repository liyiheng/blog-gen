impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        if n < 0 || n > 30 {
            return -1;
        }
        // TODO: matrix
        let a: [i32; 4] = [0, 1, 1, 2];
        if n <= 3 {
            return a[n as usize];
        }
        let mut pre_pre = 1;
        let mut pre = 2;
        while n > 3 {
            let v = pre + pre_pre;
            pre_pre = pre;
            pre = v;
            n -= 1;
        }
        return pre;
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
    assert_eq!(Solution::fib(5), 5);
}
