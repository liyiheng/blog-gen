fn main() {}
struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let indexes: Vec<_> = seats
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v == 1)
            .map(|v| v.0)
            .collect();
        let mut max_dist = 0;
        for (i, idx) in indexes.iter().enumerate() {
            let dist = if i == 0 {
                *idx
            } else {
                let span_width = idx - indexes[i - 1] - 1;
                (span_width + 1) / 2
            };
            max_dist = max_dist.max(dist);
        }
        let last_i = *indexes.last().unwrap();
        max_dist = max_dist.max(n - last_i - 1);
        max_dist as i32
    }
}
