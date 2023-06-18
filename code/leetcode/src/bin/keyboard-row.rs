struct Solution;
fn main() {
    Solution::find_words(vec![]);
}
fn row(b: u8) -> u8 {
    match b.to_ascii_lowercase() {
        b'z' | b'x' | b'c' | b'v' | b'b' | b'n' | b'm' => 0,
        b'a' | b's' | b'd' | b'f' | b'g' | b'h' | b'j' | b'k' | b'l' => 1,
        _ => 2,
    }
}
fn check(word: &[u8]) -> bool {
    if word.len() <= 1 {
        return true;
    }
    let r = row(word[0]);
    !word[1..].iter().any(|&v| row(v) != r)
}
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        words.into_iter().filter(|w| check(w.as_bytes())).collect()
    }
}
