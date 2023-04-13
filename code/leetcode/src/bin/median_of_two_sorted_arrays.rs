fn main() {
    Solution::find_median_sorted_arrays(vec![], vec![]);
}
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut a = 0;
        let mut prev_a = 0;
        for _ in 0..(total / 2 + 1) {
            prev_a = a;
            if let Some(&v1) = nums1.get(idx1) {
                if let Some(&v2) = nums2.get(idx2) {
                    a = v1.min(v2);
                    if v1 < v2 {
                        idx1 += 1;
                    } else {
                        idx2 += 1;
                    }
                } else {
                    a = v1;
                    idx1 += 1;
                }
            } else {
                a = nums2[idx2];
                idx2 += 1;
            }
        }
        // 奇数，取第 total/2 + 1 个数
        // 偶数，取第 total/2 和 total/2 + 1 个数
        if total % 2 == 0 {
            (a as f64 + prev_a as f64) / 2.0
        } else {
            a as f64
        }
    }
}
