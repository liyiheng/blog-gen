impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.as_slice();
        let line = vec![0; nums.len()];
        let mut queue = vec![];
        let mut queue_next = vec![];
        queue.push((line, 0));
        while !nums.is_empty() {
            let v = nums[0];
            while !queue.is_empty() {
                let (line, state) = queue.pop().unwrap();
                for i in 0..line.len() {
                    if is_some(state, i) {
                        continue;
                    }
                    let mut line = line.clone();
                    line[i] = v;
                    let s = set(state, i);
                    queue_next.push((line, s));
                }
            }
            queue = queue_next;
            queue_next = vec![];
            nums = &nums[1..];
        }
        queue
            .into_iter()
            .map(|a| a.0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}
fn set(state: usize, idx: usize) -> usize {
    let tmp = 1 << idx;
    state | tmp
}
fn is_some(state: usize, idx: usize) -> bool {
    let tmp = 1 << idx;
    state & tmp == tmp
}

struct Solution;

fn main() {
    Solution::permute_unique(vec![]);
}
