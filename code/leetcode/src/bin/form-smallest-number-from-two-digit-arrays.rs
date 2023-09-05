struct Solution;
fn main() {}
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        for n in nums1 {
            min1 = min1.min(n);
            s1.insert(n);
        }
        for n in nums2 {
            min2 = min2.min(n);
            s2.insert(n);
        }
        if min1 > min2 {
            std::mem::swap(&mut min1, &mut min2);
        }
        match s1.intersection(&s2).min() {
            None => min1 * 10 + min2,
            Some(v) => *v,
        }
    }
}
