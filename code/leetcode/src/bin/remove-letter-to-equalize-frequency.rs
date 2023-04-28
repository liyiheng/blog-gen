struct Solution;
fn main() {}
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counts = vec![0; 26];
        for &b in word.as_bytes().iter() {
            counts[(b - b'a') as usize] += 1;
        }
        counts.sort();
        counts.reverse();
        while *counts.last().unwrap() == 0 {
            counts.pop();
        }
        if counts.len() == 1 {
            return true;
        }
        let sum: i32 = counts.iter().sum();
        // 最后多一个 1
        // n,n,n,n,1
        if *counts.last().unwrap() == 1 && sum - 1 == counts[0] * (counts.len() as i32 - 1) {
            return true;
        }
        // 前面多 n+1
        // n+1, n,n,n,n,n
        return *counts.last().unwrap() == (counts[0] - 1) && counts[1] == counts[0] - 1;
    }
}
