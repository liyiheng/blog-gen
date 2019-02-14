impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() == 0 || words.len() == 0 {
            return vec![];
        }
        let word_len = words.first().map(|s| s.len()).unwrap_or(0);
        let mut all = std::collections::HashMap::new();
        let mut used = std::collections::HashMap::new();
        let mut size = 0;
        for w in words.into_iter() {
            size += w.len();
            let v = all.entry(w.into_bytes()).or_insert(0);
            *v += 1;
        }
        let mut result = vec![];
        let dat = s.into_bytes();
        let mut start = 0;
        'outer: while dat[start..].len() >= word_len {
            let w = &dat[start..start + word_len];
            let v = all.get_mut(&w.to_vec());
            if v.is_none() {
                let mut offset = 0;
                for (k, v) in used.iter() {
                    let vv = all.get_mut(k).unwrap();
                    *vv += v;
                    offset += v;
                }
                start -= offset * word_len;
                start += 1;
                used.clear();
                continue;
            }
            let v = v.unwrap();
            if *v == 0 {
                let mut offset = 0;
                for (k, _v) in used.iter_mut() {
                    let vv = all.get_mut(k).unwrap();
                    offset += *_v;
                    *vv += *_v;
                    *_v = 0;
                }
                start = start + 1 - offset * word_len;

                //let vv = used.get_mut(w).unwrap();
                //*v = *vv;
                //*vv = 0;
                continue;
            }
            start += word_len;
            *v -= 1;
            *used.entry(w.to_vec()).or_insert(0) += 1;
            for (_, value) in all.iter() {
                if *value > 0 {
                    continue 'outer;
                }
            }

            result.push(start as i32 - size as i32);
            for (k, v) in used.iter() {
                let vv = all.get_mut(k).unwrap();
                *vv += v;
            }
            used.clear();

            start -= size;
            start += 1;
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            "barfoofoobarthefoobarman".to_owned(),
            vec!["bar", "foo", "the"]
                .into_iter()
                .map(|s: &str| s.to_owned())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::find_substring(
            "barfoothefoobarman".to_owned(),
            vec!["foo", "bar"]
                .into_iter()
                .map(|s: &str| s.to_owned())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_owned(),
            vec!["word", "good", "best", "word"]
                .into_iter()
                .map(|s: &str| s.to_owned())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_owned(),
            vec!["word", "good", "best", "good"]
                .into_iter()
                .map(|s: &str| s.to_owned())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::find_substring(
            "abaababbaba".to_owned(),
            vec!["ab", "ba", "ab", "ba"]
                .into_iter()
                .map(|s: &str| s.to_owned())
                .collect()
        )
    );
}
