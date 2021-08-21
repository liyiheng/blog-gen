#![allow(dead_code)]

struct Solution;
fn main() {}

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let dat = unsafe { s.as_bytes_mut() };
        let mut start = 0;
        let mut end = dat.len() - 1;
        let is_vowel = |b: u8| -> bool {
            match b {
                b'a' | b'e' | b'i' | b'o' | b'u' => return true,
                b'A' | b'E' | b'I' | b'O' | b'U' => return true,
                _ => return false,
            }
        };
        while start < end {
            while start < end && !is_vowel(dat[start]) {
                start += 1;
            }
            while start < end && !is_vowel(dat[end]) {
                end -= 1;
            }
            if start < end {
                dat.swap(start, end);
                start += 1;
                if end > 0 {
                    end -= 1;
                }
            }
        }
        s
    }
}
