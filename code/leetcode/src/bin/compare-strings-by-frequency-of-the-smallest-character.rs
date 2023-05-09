struct Solution;
fn main() {
    // 定义一个函数 f(s)，统计 s  中（按字典序比较）最小字母的出现频次 ，其中 s 是一个非空字符串。
    // 例如，若 s = "dcce"，那么 f(s) = 2，因为字典序最小字母是 "c"，它出现了 2 次。
    // 现在，给你两个字符串数组待查表 queries 和词汇表 words 。
    // 对于每次查询 queries[i] ，需统计 words 中满足 f(queries[i]) < f(W) 的 词的数目 ，
    // W 表示词汇表 words 中的每个词。
    // 请你返回一个整数数组 answer 作为答案，其中每个 answer[i] 是第 i 次查询的结果。*/
}

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let queries: Vec<i32> = queries.into_iter().map(|s| count_feq(s.as_str())).collect();
        let mut words: Vec<i32> = words.into_iter().map(|s| count_feq(s.as_str())).collect();
        words.sort();
        let mut ans = vec![];
        for q in queries.iter() {
            let i = match words.binary_search(q) {
                Ok(mut idx) => {
                    while idx < words.len() {
                        if words[idx] != *q {
                            break;
                        }
                        idx += 1;
                    }
                    idx
                }
                Err(insert_idx) => insert_idx,
            };
            ans.push((words.len() - i) as i32);
        }
        ans
    }
}
fn count_feq(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let dat = s.as_bytes();
    let mut cur_c = dat[0];
    let mut ans = 0;
    for &b in dat.iter() {
        if b == cur_c {
            ans += 1;
        } else if b < cur_c {
            cur_c = b;
            ans = 1;
        }
    }
    ans
}
