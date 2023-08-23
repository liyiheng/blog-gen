fn main() {}
struct Solution;
impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut details: Vec<_> = indices
            .into_iter()
            .zip(sources.into_iter())
            .zip(targets.into_iter())
            .map(|((i, s), t)| (i as usize, s, t))
            .collect();
        details.sort_by_cached_key(|e| e.0);
        let mut idx_hit = std::collections::HashMap::new();
        for (idx, src, t) in details.into_iter() {
            let tmp = &s[idx..];
            if tmp.starts_with(src.as_str()) {
                idx_hit.insert(idx, (src, t));
            }
        }
        let mut ans = vec![];
        let mut i = 0;
        while i < s.len() {
            if let Some((src, target)) = idx_hit.get(&i) {
                let mut tt = target.as_bytes().to_vec();
                ans.append(&mut tt);
                i += src.len();
                continue;
            }
            ans.push(s.as_bytes()[i]);
            i += 1;
        }
        String::from_utf8(ans).unwrap()
    }
}
