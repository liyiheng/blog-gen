struct Solution;
pub fn main() {
    Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string());
}
impl Solution {
    pub fn tmp(text: &[u8]) -> i32 {
        if text.is_empty() {
            return 0;
        }
        let mut lo = 1;
        let mut hi = text.len() - 1;
        while lo <= hi {
            let head = &text[..lo];
            let tail = &text[hi..];
            if head == tail {
                return 2 + Solution::tmp(&text[lo..hi]);
            }
            lo += 1;
            hi -= 1;
        }
        1
    }
    pub fn longest_decomposition(text: String) -> i32 {
        Solution::tmp(text.as_bytes())
    }
}
