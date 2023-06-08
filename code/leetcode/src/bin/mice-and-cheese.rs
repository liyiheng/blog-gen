struct Solution;
fn main() {}
impl Solution {
    pub fn mice_and_cheese_v1(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut tmp: Vec<(i32, i32, i32)> = reward1
            .into_iter()
            .zip(reward2.into_iter())
            .map(|(r1, r2)| (r1 - r2, r1, r2))
            .collect();
        tmp.sort_by_key(|v| v.0);
        tmp.reverse();
        let k = k as usize;
        tmp.into_iter()
            .enumerate()
            .map(|(i, (_, r1, r2))| if i < k { r1 } else { r2 })
            .sum()
    }
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let base: i32 = reward2.iter().sum();
        let mut tmp: Vec<i32> = reward1
            .into_iter()
            .zip(reward2.into_iter())
            .map(|(r1, r2)| r1 - r2)
            .collect();
        tmp.sort();
        let k = tmp.len() - k as usize;
        tmp[k..].iter().sum::<i32>() + base
    }
}
