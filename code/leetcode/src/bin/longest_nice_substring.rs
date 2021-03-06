impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        if s.len() == 1 {
            return String::default();
        }
        use std::collections::HashMap;
        let raw: Vec<char> = s.chars().collect();
        let mut counts = HashMap::<char, u32>::new();
        for &b in raw.iter() {
            counts.entry(b).and_modify(|c| *c += 1).or_insert(1);
        }
        let mut matrix = vec![vec![false; raw.len() + 1]; raw.len() + 1];
        for start in 0..raw.len() - 1 {
            if start > 0 {
                counts.entry(raw[start - 1]).and_modify(|c| *c -= 1);
            }

            let mut counts2 = HashMap::<char, u32>::new();
            for end in (start + 1..=raw.len()).rev() {
                if end < raw.len() {
                    counts2.entry(raw[end]).and_modify(|c| *c += 1).or_insert(1);
                }
                let mut ok = true;
                for (k, v) in counts.iter() {
                    let another = if k.is_ascii_lowercase() {
                        k.to_ascii_uppercase()
                    } else {
                        k.to_ascii_lowercase()
                    };
                    let c1 = v - counts2.get(&k).cloned().unwrap_or_default();
                    let c2 = counts.get(&another).cloned().unwrap_or_default()
                        - counts2.get(&another).cloned().unwrap_or_default();
                    if c1 == c2 {
                        continue;
                    }
                    if c1 == 0 || c2 == 0 {
                        ok = false;
                        break;
                    }
                }
                matrix[start][end] = ok;
            }
        }
        let mut max = 0;
        let mut start = 0;
        for i in 0..matrix.len() {
            for j in i + 1..matrix.len() {
                if !matrix[i][j] {
                    continue;
                }
                if j - i > max {
                    max = j - i;
                    start = i;
                }
            }
        }
        let sub = &raw[start..start + max];
        sub.iter().collect()
    }
}

struct Solution;
fn main() {
    println!(
        "{}",
        Solution::longest_nice_substring("YazaAay".to_string())
    );

    println!("{}", Solution::longest_nice_substring("Bb".to_string()));
    println!("{}", Solution::longest_nice_substring("c".to_string()));
    println!("{}", Solution::longest_nice_substring("dDzeE".to_string()));
}
