impl Solution {
    pub fn decode_string(s: String) -> String {
        let dat = s.into_bytes();
        let mut cnt = 0;
        let mut counts = vec![];
        let mut bufs = vec![];
        for b in dat {
            match b {
                b'0'...b'9' => {
                    cnt = cnt * 10 + (b - b'0') as i32;
                }
                b'[' => {
                    counts.push(cnt);
                    cnt = 0;
                    bufs.push(String::new());
                }
                b']' => {
                    let last = bufs.pop().unwrap_or("fck".to_string());
                    let mut pre_last = bufs.pop().unwrap_or_default();
                    let mut n = counts.pop().unwrap_or(0);
                    let mut repeat = String::new();
                    while n > 0 {
                        repeat.push_str(&last);
                        n -= 1;
                    }
                    pre_last.push_str(&repeat);
                    bufs.push(pre_last);
                }
                _ => {
                    if bufs.len() == 0 {
                        bufs.push(String::new());
                    }
                    let last = bufs.last_mut().unwrap();
                    last.push(b as char);
                }
            }
        }
        let mut result = String::new();
        for s in bufs {
            result.push_str(&s);
        }
        result
    }
}

struct Solution {}

macro_rules! S {
    ($s:expr) => {
        Solution::decode_string($s.to_string())
    };
}
fn main() {
    assert_eq!("aaabcbc", S!("3[a]2[bc]"));
    assert_eq!("abcabccdcdcdef", S!("2[abc]3[cd]ef"));
    assert_eq!("accaccacc", S!("3[a2[c]]"));
    assert_eq!("hellohellohello", S!("3[hello]"));
    assert_eq!("aaabFFFFcbFFFFc", S!("3[a]2[b4[F]c]"));
}
