struct Solution;
fn main() {
    dbg!(Solution::find_kth_bit(3, 1));
    dbg!(Solution::find_kth_bit(4, 11));
    dbg!(Solution::find_kth_bit(1, 1));
    dbg!(Solution::find_kth_bit(2, 3));
}

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if k == 1 {
            return '0';
        }
        let len_n = 2i32.pow(n as u32) - 1;
        if k == len_n / 2 + 1 {
            return '1';
        }
        if k < len_n / 2 + 1 {
            return Solution::find_kth_bit(n - 1, k);
        }
        let c = Solution::find_kth_bit(n - 1, len_n - k + 1);
        if c == '0' {
            '1'
        } else {
            '0'
        }
    }
}
