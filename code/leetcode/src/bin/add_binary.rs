impl Solution {
    fn add(a: u8, b: u8, remaind: u8) -> (u8, u8) {
        let s = a - b'0' + b - b'0' + remaind - b'0';
        match s {
            0 => (b'0', b'0'),
            1 => (b'1', b'0'),
            2 => (b'0', b'1'),
            3 => (b'1', b'1'),
            _ => unreachable!(),
        }
    }

    pub fn add_binary(a: String, b: String) -> String {
        let l = if a.len() > b.len() { a.len() } else { b.len() };
        let a: Vec<u8> = a.bytes().collect();
        let b: Vec<u8> = b.bytes().collect();
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let diff = a.len() - b.len();

        let mut c = vec![b'0'; l];
        let mut remaind = b'0';
        for i in (0..l).rev() {
            let d = a.get(i).map(|v| *v).unwrap_or(b'0');
            let e = if i >= diff {
                b.get(i - diff).map(|v| *v).unwrap_or(b'0')
            } else {
                b'0'
            };
            let (f, x) = Solution::add(d, e, remaind);
            c[i] = f;
            remaind = x;
        }
        if remaind == b'1' {
            c.insert(0, b'1');
        }
        String::from_utf8(c).unwrap()
    }
}

struct Solution;
fn check(a: &str, b: &str, c: &str) {
    assert_eq!(
        a,
        Solution::add_binary(b.to_string(), c.to_string()).as_str()
    );
}
fn main() {
    check("111", "1", "110");
    check("110", "11", "11");
    check("10101", "1010", "1011");
}
