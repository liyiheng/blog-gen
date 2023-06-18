struct Solution;
fn main() {
    Solution::find_longest_word("".to_owned(), vec![]);
}
fn check(s: &[u8], word: &[u8]) -> bool {
    if word.len() >= s.len() {
        return word.eq(s);
    }
    let mut s = s.iter();
    for b in word.iter() {
        loop {
            if let Some(v) = s.next() {
                if v == b {
                    break;
                }
                continue;
            } else {
                return false;
            }
        }
    }
    true
}
impl Solution {
    pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
        dictionary.sort_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                b.len().cmp(&a.len())
            }
        });
        let s = s.as_bytes();
        dictionary
            .into_iter()
            .find(|v| check(s, v.as_bytes()))
            .unwrap_or_default()
    }
}
