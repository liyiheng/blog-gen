struct Solution;
fn main() {}
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        if croak_of_frogs.len() % 5 != 0 {
            return -1;
        }
        let mut cnt_c = 0;
        let mut cnt_r = 0;
        let mut cnt_o = 0;
        let mut cnt_a = 0;
        let mut cnt_k = 0;
        for &b in croak_of_frogs.as_bytes() {
            match b {
                b'c' => cnt_c += 1,
                b'r' => cnt_r += 1,
                b'o' => cnt_o += 1,
                b'a' => cnt_a += 1,
                b'k' => cnt_k += 1,
                _ => unreachable!(),
            }
        }
        let ok = cnt_c == cnt_r && cnt_r == cnt_o && cnt_o == cnt_a && cnt_a == cnt_k;
        if !ok {
            return -1;
        }
        let mut cnt_c = 0;
        let mut cnt_r = 0;
        let mut cnt_o = 0;
        let mut cnt_a = 0;
        let mut ans = 0;
        for &b in croak_of_frogs.as_bytes() {
            match b {
                b'c' => cnt_c += 1,
                b'r' => cnt_r += 1,
                b'o' => cnt_o += 1,
                b'a' => cnt_a += 1,
                b'k' => {
                    cnt_c -= 1;
                    cnt_r -= 1;
                    cnt_o -= 1;
                    cnt_a -= 1;
                }
                _ => unreachable!(),
            }
            if cnt_c < cnt_r || cnt_r < cnt_o || cnt_o < cnt_a || cnt_a < 0 {
                return -1;
            }
            ans = ans.max(cnt_c);
        }
        ans
    }
}
