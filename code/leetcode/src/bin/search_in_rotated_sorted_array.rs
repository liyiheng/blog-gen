impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        return Solution::bin_search(&nums, 0, nums.len() as i32 - 1, target);
    }
    fn bin_search(nums: &Vec<i32>, lo: i32, hi: i32, target: i32) -> i32 {
        if lo >= hi {
            if nums[lo as usize] == target {
                return lo;
            }
            return -1;
        }
        let a = nums[lo as usize];
        if a == target {
            return lo;
        }
        let b = nums[hi as usize];
        if b == target {
            return hi;
        }
        let mid = lo + (hi - lo) / 2;
        let mid_v = nums[mid as usize];
        if mid_v == target {
            return mid;
        }

        if a <= mid_v && mid_v <= b {
            // Normal binary search
            if target < mid_v {
                return Solution::bin_search(nums, lo, mid - 1, target);
            }
            return Solution::bin_search(nums, mid, hi - 1, target);
        }
        if a <= mid_v {
            if a < target && target < mid_v {
                return Solution::bin_search(nums, lo, mid - 1, target);
            }
            return Solution::bin_search(nums, mid, hi - 1, target);
        }
        if target > mid_v {
            if target > a {
                return Solution::bin_search(nums, lo + 1, mid, target);
            }
            return Solution::bin_search(nums, mid, hi - 1, target);
        }
        if target < a && target > b {
            return Solution::bin_search(nums, lo + 1, mid, target);
        }
        return Solution::bin_search(nums, lo, mid, target);
    }
}

struct Solution;

fn main() {
    let v = Solution::search(vec![5, 6, 7, 8, 9, 1, 2, 3, 4], 3);
    assert_eq!(v, 7);
    let v = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
    assert_eq!(v, 4);
    let v = Solution::search(vec![1, 2], 0);
    assert_eq!(v, -1);
    let v = Solution::search(vec![2, 1], 0);
    assert_eq!(v, -1);
    let v = Solution::search(vec![5, 1, 3], 5);
    assert_eq!(v, 0);
    let v = Solution::search(vec![5, 1, 2, 3, 4], 1);
    assert_eq!(v, 1);
    let v = Solution::search(vec![8, 9, 2, 3, 4], 9);
    assert_eq!(v, 1);
}
