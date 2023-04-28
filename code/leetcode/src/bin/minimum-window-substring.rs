struct Solution;
fn main() {
    dbg!(Solution::min_window(
        "ADOBECODEBANC".to_string(),
        "ABC".to_string()
    ));
}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let mut state = HashMap::new();
        for &b in t.as_bytes().iter() {
            *state.entry(b).or_insert(0) += 1;
        }
        let set = state.clone();
        let s = s.as_bytes();
        let mut min_dist = i32::MAX as usize;
        let mut min_dist_start = 0;
        let mut start = 0;
        let mut end = 0;
        while end <= s.len() {
            let contains = state.values().filter(|&v| *v > 0).count() == 0;
            if contains {
                if min_dist > end - start {
                    min_dist = end - start;
                    min_dist_start = start;
                }
                start += 1;
                let c = s[start - 1];
                if !set.contains_key(&c) {
                    // 出窗口的字符不在 t，继续往后
                    continue;
                }
                *state.entry(c).or_default() += 1;
            } else {
                end += 1;
                if end > s.len() {
                    break;
                }
                let c = s[end - 1];
                if !set.contains_key(&c) {
                    // 当前字符不在 t，继续往后
                    continue;
                }
                if let Some(n) = state.get_mut(&c) {
                    *n -= 1;
                }
            }
            let contains = state.values().filter(|&v| *v > 0).count() == 0;
            if contains {
                if min_dist > end - start {
                    min_dist = end - start;
                    min_dist_start = start;
                }
            }
        }
        if min_dist + min_dist_start > s.len() {
            return "".to_string();
        }
        println!("start:{}, len:{}", min_dist_start, min_dist);
        let ans = &s[min_dist_start..min_dist_start + min_dist];
        String::from_utf8(ans.to_vec()).unwrap()
    }
}
