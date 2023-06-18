struct Solution;
fn main() {
    dbg!(Solution::is_perfect_square(16));
    dbg!(Solution::is_perfect_square(123 * 123));
    dbg!(Solution::is_perfect_square(i32::MAX - 10));
    dbg!(Solution::is_perfect_square((MAX - 5).pow(2)));
    dbg!(Solution::is_perfect_square((MAX - 9).pow(2)));
    dbg!(Solution::is_perfect_square(81));
    dbg!(Solution::is_perfect_square(18));
}
// ((1<<31) -1).sqrt().floor()
const MAX: i32 = 46340;
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut lo = 1;
        let mut hi = MAX;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            let v = mid.pow(2);
            if v > num && (mid - 1).pow(2) < num {
                return false;
            }
            if v < num && (mid + 1).pow(2) > num {
                return false;
            }
            if v == num {
                return true;
            }
            if v > num {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        false
    }
}
