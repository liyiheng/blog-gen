struct Solution;
fn main() {
    let check = |n| {
        let mut data: Vec<i32> = (3..n).collect();
        for _ in 0..n {
            let v = data.remove(0);
            data.push(v);
            dbg!(Solution::find_min(data.clone()));
        }
    };
    check(20);
    check(25);
}
use std::cmp::Ordering;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let first = nums[0];
        let last = *nums.last().unwrap();
        if last > first {
            return nums[0];
        }
        let tmp: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
        let idx = tmp
            .binary_search_by(|&(i, &v)| {
                if i == 0 {
                    return Ordering::Less;
                }
                // [4,5,6,1,2,3]
                // [6,1] 局部递减，命中目标
                if nums[i - 1] > v {
                    return Ordering::Equal;
                }
                if v < first {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap();
        nums[idx]
    }
}
