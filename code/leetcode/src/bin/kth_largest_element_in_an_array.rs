#![allow(dead_code)]
fn main() {
    println!("Kth:{}", Solution::find_kth_largest(vec![3, 1, 2, 4], 2));
    println!("Kth:{}", Solution::find_kth_largest(vec![4], 1));
}
mod min_heap;
use crate::min_heap::MinHeap;
struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let l = nums.len() as i32;
        let smallest = l / 2 <= k;
        let cap = if smallest { l - k + 1 } else { k };

        let mut heap = MinHeap::new(cap as usize);
        if smallest {
            for i in nums {
                heap.add(-i);
            }
            return -heap.peak().unwrap_or(0);
        }
        for i in nums {
            heap.add(i);
        }
        return heap.peak().unwrap_or(0);
    }
}
