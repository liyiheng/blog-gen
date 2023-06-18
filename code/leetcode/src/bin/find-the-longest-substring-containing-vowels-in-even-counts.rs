struct Solution;
fn main() {
    dbg!(Solution::find_the_longest_substring(
        "leetcodeisgreat".to_owned()
    ));
    dbg!(Solution::find_the_longest_substring(
        "eleetminicoworoep".to_owned()
    ));
}
fn print_state(v: u8) {
    let mut s = String::new();
    for i in 0..5 {
        let mask = 1 << i;
        if v & mask == mask {
            s.push('奇');
        } else {
            s.push('偶');
        }
    }
    println!("{} {}", s, v);
}
fn to_idx(b: u8) -> Option<usize> {
    match b {
        b'a' => Some(0),
        b'e' => Some(1),
        b'i' => Some(2),
        b'o' => Some(3),
        b'u' => Some(4),
        _ => None,
    }
}
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut a: HashMap<u8, usize> = HashMap::new();
        let l = s.len();
        let dat = s.as_bytes();
        let mut prefix = vec![0u8; l];
        let mut ans = 0;
        a.insert(0, 0);
        for (i, &b) in dat.iter().enumerate() {
            if i > 0 {
                prefix[i] = prefix[i - 1];
            }
            if let Some(idx) = to_idx(b) {
                prefix[i] ^= 1 << idx;
            }
            if let Some(prev_i) = a.get(&prefix[i]) {
                ans = ans.max(i + 1 - prev_i);
            } else {
                a.insert(prefix[i], i + 1);
            }
        }
        if cfg!(debug_assertions) {
            println!("\t   a e i o u");
            for (i, s) in prefix.iter().enumerate() {
                print!("{}\t{} ", i, dat[i] as char);
                print_state(*s);
            }
        }
        ans as i32
    }
}
