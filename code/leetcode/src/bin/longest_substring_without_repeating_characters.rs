impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = std::collections::HashMap::new();
        let mut max = 0;
        let mut start = 0;
        let dat = s.into_bytes();
        for i in 0..dat.len() {
            let b = dat[i];
            let v = set.get(&b);
            if v.is_none() {
                set.insert(b, i);
                continue;
            }
            let prev = *v.unwrap();
            if prev < start {
                set.insert(b, i);
                continue;
            }
            if i - start > max {
                max = i - start
            }
            start = prev + 1;
            set.insert(b, i);
        }
        let l = dat.len() - start;
        if l > max {
            max = l;
        }
        max as i32
    }
}

struct Solution {}

fn main() {
    let f = |s: &str| -> i32 { Solution::length_of_longest_substring(s.to_string()) };
    assert_eq!(3, f("abcabcbb"));
    assert_eq!(3, f("dvdf"));
    assert_eq!(1, f("bbbbb"));
    assert_eq!(3, f("pwwkew"));
}
