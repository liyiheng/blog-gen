struct Solution;
fn main() {
    Solution::longest_str_chain(
        ["a", "b", "ba", "bca", "bda", "bdca"]
            .iter()
            .map(|v| v.to_string())
            .collect(),
    );
    Solution::longest_str_chain(
        ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
            .iter()
            .map(|v| v.to_string())
            .collect(),
    );
}

fn check(a: &[u8], b: &[u8]) -> bool {
    if a.len() + 1 != b.len() {
        return false;
    }
    let al = a.len();
    let mut offset = 0;
    for i in 0..al {
        if a[i] == b[i + offset] {
            continue;
        }
        if offset > 0 {
            return false;
        }
        offset += 1;
        if a[i] != b[i + offset] {
            return false;
        }
    }
    true
}
fn depth(relations: &[Vec<usize>], cur_node: usize) -> i32 {
    relations[cur_node]
        .iter()
        .map(|&child| depth(relations, child))
        .max()
        .unwrap_or_default()
        + 1
}
impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|v| v.len());
        // relations[i] 存储节点 i 的所有子节点
        let mut relations = vec![vec![]; words.len()];
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if check(words[i].as_bytes(), words[j].as_bytes()) {
                    relations[i].push(j);
                }
            }
        }
        (0..words.len())
            .map(|root| depth(&relations, root))
            .max()
            .unwrap_or_default()
    }
}
