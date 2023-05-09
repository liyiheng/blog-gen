struct Solution;
fn main() {}
impl Solution {
    pub fn count_time(time: String) -> i32 {
        let h1 = time.as_bytes()[0];
        let h2 = time.as_bytes()[1];
        let mut h_count = 1;
        if h1 == b'?' && h2 == b'?' {
            h_count = 24;
        } else if h1 == b'?' {
            match h2 {
                b'0' | b'1' | b'2' | b'3' => h_count = 3, // 03, 13, 23
                _ => h_count = 2,
            }
        } else if h2 == b'?' {
            match h1 {
                b'0' | b'1' => h_count = 10, // 03, 13, 23
                _ => h_count = 4,
            }
        }
        let m1 = time.as_bytes()[3];
        let m2 = time.as_bytes()[4];
        let mut m_count = 1;
        if m1 == b'?' && m2 == b'?' {
            m_count = 60;
        } else if m1 == b'?' {
            m_count = 6;
        } else if m2 == b'?' {
            m_count = 10;
        }
        h_count * m_count
    }
}
