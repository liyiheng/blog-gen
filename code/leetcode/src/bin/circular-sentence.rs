struct Solution;
fn main() {
    dbg!(Solution::is_circular_sentence("".to_owned()));
}
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<_> = sentence
            .split_whitespace()
            .map(|s| (s.as_bytes()[0], *s.as_bytes().last().unwrap()))
            .collect();
        let l = words.len();
        for i in 0..l {
            let cur = words[i];
            let next = words[(i + 1) % l];
            if cur.1 != next.0 {
                return false;
            }
        }
        true
    }
}
