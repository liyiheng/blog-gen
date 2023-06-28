struct Solution;
fn main() {
    dbg!(Solution::circular_game_losers(5, 6));
}
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut counts = vec![0; n];
        let mut i = 0;
        let mut step = k;
        while counts[i] == 0 {
            counts[i] += 1;
            i = (i + step) % n;
            step += k;
        }
        counts
            .into_iter()
            .enumerate()
            .filter(|&(_, c)| c == 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}
