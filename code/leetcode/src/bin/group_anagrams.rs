impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut m: HashMap<_, Vec<String>> = HashMap::new();
        for s in strs {
            let mut k = [0; 26];
            for c in s.as_bytes() {
                k[(c - b'a') as usize] += 1;
            }
            m.entry(k).or_default().push(s);
        }
        m.into_iter().map(|(_, v)| v).collect()
    }
    #[allow(dead_code)]
    pub fn group_anagrams_slow(strs: Vec<String>) -> Vec<Vec<String>> {
        let l = strs.len();
        let mut indexes = vec![];
        for i in 0..l {
            indexes.push(i);
        }
        let sorted: Vec<Vec<u8>> = strs
            .iter()
            .map(|s| {
                let mut dat = s.clone().into_bytes();
                dat.sort();
                dat
            })
            .collect();
        let mut result: Vec<Vec<usize>> = vec![];
        'outer: for i in 0..l {
            let v = &sorted[i];
            for s in result.iter_mut() {
                if sorted[s[0]] == *v {
                    s.push(i);
                    continue 'outer;
                }
            }
            result.push(vec![i]);
        }
        result
            .into_iter()
            .map(|arr| arr.into_iter().map(|i| strs[i].clone()).collect())
            .collect()
    }
}

struct Solution;

fn main() {
    let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let output = Solution::group_anagrams(input);
    println!("{:?}", output);
}
