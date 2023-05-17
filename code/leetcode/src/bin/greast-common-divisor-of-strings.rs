struct Solution;
fn main() {
    dbg!(gcd(17 * 12, 17 * 100));
    dbg!(gcd(17 * 12, 17 * 13));
    dbg!(gcd(1, 100));
    dbg!(gcd(15, 100));
    dbg!(gcd(50, 100));
    Solution::gcd_of_strings("".to_string(), "".to_string());
}
fn gcd(a: usize, b: usize) -> usize {
    let mut l = a.min(b);
    let mut s = a.max(b);
    while l % s != 0 {
        let r = l % s;
        l = s;
        s = r;
    }
    s
}
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let l1 = str1.len();
        let l2 = str2.len();
        let a = str1.clone() + str2.as_str();
        let b = str2.clone() + str1.as_str();
        if a != b {
            return String::new();
        }
        let l = gcd(l1, l2);
        let data = &str1.as_bytes()[..l];
        String::from_utf8(data.to_vec()).unwrap()
    }
}
