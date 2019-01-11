#![allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();
        let l = nums.len();
        let mut i = 0;
        while i < l {
            let cur = nums[i];
            let target = -cur;

            let mut start = i + 1;
            let mut end = l - 1;
            while start < end {
                let a = nums[start];
                let b = nums[end];
                if b < target / 2 {
                    break;
                }
                let sum = a + b;
                if sum > target {
                    while end - 1 > start && nums[end] == nums[end - 1] {
                        end -= 1;
                    }
                    end -= 1;
                    continue;
                }
                if sum < target {
                    while start + 1 < end && nums[start] == nums[start + 1] {
                        start += 1;
                    }
                    start += 1;
                    continue;
                }
                result.push(vec![cur, a, b]);
                while start + 1 < end && nums[start] == nums[start + 1] {
                    start += 1;
                }
                start += 1;
            }
            while i + 1 < l && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        result
    }
}

struct Solution {}

fn main() {
    println!("hello");
}
