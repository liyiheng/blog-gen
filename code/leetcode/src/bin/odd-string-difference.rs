struct Solution;
fn main() {}
impl Solution {
    pub fn odd_string(mut words: Vec<String>) -> String {
        let to_diff = |s: &str| -> Vec<i32> {
            let s = s.as_bytes();
            let mut d = vec![];
            for i in 1..s.len() {
                let v = s[i] as i32 - s[i - 1] as i32;
                d.push(v);
            }
            d
        };
        let a = words.pop().unwrap();
        let b = words.pop().unwrap();
        let first = to_diff(a.as_str());
        let second = to_diff(b.as_str());
        if first == second {
            for s in words.into_iter() {
                let diff = to_diff(s.as_str());
                if diff != first {
                    return s;
                }
            }
        } else {
            for s in words.into_iter() {
                let diff = to_diff(s.as_str());
                if diff == first {
                    return b;
                } else {
                    return a;
                }
            }
        }
        String::new()
    }
}
