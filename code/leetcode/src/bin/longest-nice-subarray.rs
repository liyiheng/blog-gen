struct Solution;
fn main() {
    dbg!(Solution::longest_nice_subarray(vec![
        135745088, 609245787, 16, 2048, 2097152
    ]));
    dbg!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]));
    dbg!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
}
fn scan<F: FnMut(usize, i32)>(n: i32, mut f: F) {
    for i in 0..30 {
        let mask = 1 << i;
        let zero = mask & n == 0;
        if zero {
            f(i, 0);
        } else {
            f(i, 1);
        }
    }
}
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ones = vec![0; 31];
        let mut ans = 0;
        let mut start = 0;
        let mut end = 0;
        while start <= end && start < nums.len() {
            let ok = ones.iter().find(|&&v| v > 1).is_none();
            if ok {
                ans = ans.max(end - start);
                end += 1;
                if end - 1 == nums.len() {
                    break;
                }
                let n = nums[end - 1];
                scan(n, |i, one| {
                    ones[i] += one;
                });
                continue;
            }
            let n = nums[start];
            start += 1;
            scan(n, |i, one| {
                ones[i] -= one;
            });
        }
        ans as i32
    }
}
