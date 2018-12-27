fn main() {
    let r = Solution::search_range(vec![], 0);
    println!("{:?}", r);
}

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hit: i32 = -1;
        let mut high = nums.len() as i32 - 1;
        let mut low = 0;
        while low <= high {
            let mid = (low + high) / 2;
            if target == nums[mid as usize] {
                hit = mid as i32;
                break;
            }
            if target > nums[mid as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        if hit == -1 {
            return vec![-1, -1];
        }
        let mut start = hit - 1;
        let mut end = hit + 1;
        while end < nums.len() as i32 {
            if nums[end as usize] > target {
                break;
            }
            end += 1;
        }
        while start >= 0 {
            if nums[start as usize] < target {
                break;
            }

            start -= 1;
        }

        vec![start + 1, end - 1]
    }
}
