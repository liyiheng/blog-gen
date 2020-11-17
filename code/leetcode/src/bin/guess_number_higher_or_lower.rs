#![allow(non_snake_case)]
unsafe fn guess(num: i32) -> i32 {
    if PICK == num {
        0
    } else if num < PICK {
        1
    } else {
        -1
    }
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut hi = n;
        let mut lo = 1;
        let mut mid = hi / 2 + 1;
        while lo < hi {
            let f = guess(mid);
            if f == 0 {
                return mid;
            } else if f < 0 {
                hi = mid - 1;
                mid = ((mid as u32 + lo as u32) / 2) as i32;
            } else {
                lo = mid + 1;
                mid = ((mid as u32 + hi as u32) / 2) as i32;
            }
        }
        lo
    }
}
struct Solution;
static mut PICK: i32 = 6;
fn main() {
    for n in 1000..2000 {
        for p in 1..=n {
            unsafe {
                PICK = p;
                assert_eq!(p, Solution::guessNumber(n));
            }
        }
    }
}
