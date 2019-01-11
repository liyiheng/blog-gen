#![allow(dead_code)]
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();
        let l = nums.len();
        let mut i = 0;
        while i < l {
            let first = nums[i];
            if first > target / 4 {
                break;
            }
            let mut j = i + 1;
            while j < l {
                let second = nums[j];
                if second > (target - first) / 3 {
                    break;
                }
                let mut start = j + 1;
                let mut end = l - 1;
                while start < end {
                    let a = nums[start];
                    let b = nums[end];
                    let sum = a + b + second + first;
                    if sum > target {
                        while start < end && nums[end - 1] == nums[end] {
                            end -= 1;
                        }
                        end -= 1;
                        continue;
                    }
                    if sum == target {
                        result.push(vec![first, second, a, b]);
                    }
                    while start < end && nums[start + 1] == nums[start] {
                        start += 1;
                    }
                    start += 1;
                }
                while j + 1 < l && nums[j] == nums[j + 1] {
                    j += 1;
                }
                j += 1;
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
