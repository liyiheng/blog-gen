struct Solution;
fn main() {
    let a = Solution::can_make_pali_queries;
    _ = a;
}
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut counter = 0u32;
        let mut counters = vec![counter];
        for &b in s.as_bytes().iter() {
            counter ^= 1 << (b - b'a');
            counters.push(counter);
        }
        let check = |start: usize, end: usize, k: i32| -> bool {
            let a = counters[start];
            let b = counters[end + 1];
            let c = a ^ b;
            c.count_ones() / 2 <= k as u32
        };
        queries
            .into_iter()
            .map(|q| check(q[0] as usize, q[1] as usize, q[2]))
            .collect()
    }
}
