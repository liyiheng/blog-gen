impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let data = s.as_bytes();
        let l = data.len();

        let (mut a, mut b) = (0, 1);
        for i in 0..l {
            // <-- (i) -->
            if i > 0 {
                let mut lo = i;
                let mut hi = i;
                while hi < l {
                    if data[lo] == data[hi] {
                        if hi - lo + 1 > b - a {
                            a = lo;
                            b = hi + 1;
                        }
                        if lo == 0 {
                            break;
                        }
                        lo -= 1;
                        hi += 1;
                    } else {
                        break;
                    }
                }
            }
            // <-- (i,i+1) -->
            let mut lo = i;
            let mut hi = i + 1;
            while hi < l {
                if data[lo] == data[hi] {
                    if hi - lo + 1 > b - a {
                        a = lo;
                        b = hi + 1;
                    }
                    if lo == 0 {
                        break;
                    }
                    lo -= 1;
                    hi += 1;
                } else {
                    break;
                }
            }
        }
        String::from_utf8((&data[a..b]).to_vec()).unwrap()
    }
}
struct Solution;
fn main() {
    dbg!(Solution::longest_palindrome("babad".to_string()));
    dbg!(Solution::longest_palindrome("a".to_string()));
    dbg!(Solution::longest_palindrome("aa".to_string()));
    dbg!(Solution::longest_palindrome("aaaa".to_string()));
    dbg!(Solution::longest_palindrome("aaa".to_string()));
    dbg!(Solution::longest_palindrome("abba".to_string()));
}
