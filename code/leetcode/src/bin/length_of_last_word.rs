impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut l = 0;
        for b in s.into_bytes().into_iter().rev() {
            if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') {
                l += 1;
            } else if l > 0 {
                break;
            }
        }
        l
    }
}

struct Solution;

fn main() {
    Solution::length_of_last_word("".to_owned());
}
