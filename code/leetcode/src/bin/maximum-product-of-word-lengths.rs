struct Solution;
fn main() {}
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let values: Vec<(u32, &str)> = words
            .iter()
            .map(|s| {
                let mut v: u32 = 0;
                for &b in s.as_bytes() {
                    let offset = b - b'a';
                    let mask = 1 << offset;
                    v |= mask;
                }
                (v, s.as_str())
            })
            .collect();
        let mut max = 0;
        for (stat1, word1) in values.iter() {
            for (stat2, word2) in values.iter() {
                if stat1 & stat2 != 0 {
                    continue;
                }
                max = max.max(word1.len() * word2.len());
            }
        }
        max as i32
    }
}
