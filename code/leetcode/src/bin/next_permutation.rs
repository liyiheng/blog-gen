impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let l = nums.len();
        let mut i = l - 1;
        loop {
            let cur = nums[i];
            let mut j = i;
            let mut min = std::i32::MAX;
            let mut min_j = l;
            while j < l - 1 {
                j += 1;
                if nums[j] > cur && nums[j] < min {
                    min = nums[j];
                    min_j = j;
                }
            }
            if min_j >= l {
                if i == 0 {
                    nums.sort();
                    break;
                }
                i -= 1;
                continue;
            }

            nums[i] = nums[min_j];
            nums[min_j] = cur;
            let tail = &mut nums[i + 1..];
            tail.sort();
            return;
        }
    }
}

struct Solution;
fn main() {
    let mut a = vec![1, 2, 3];
    for _ in 0..8 {
        Solution::next_permutation(&mut a);
        println!("{:?}", a);
    }
}
