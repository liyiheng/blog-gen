impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let l = code.len();
        let mut out = vec![0; l];
        if k > 0 {
            let sum = code[1..k as usize + 1].iter().sum();
            out[0] = sum;
            for i in 1..l {
                let mut prev = out[i - 1];
                prev -= code[i];
                prev += code[(i + k as usize) % l];
                out[i] = prev;
            }
        } else if k < 0 {
            let sum = code[l + k as usize..l].iter().sum();
            out[0] = sum;
            for i in 1..l {
                let mut prev = out[i - 1];
                prev += code[i - 1];
                prev -= code[(l + i + k as usize - 1) % l];
                out[i] = prev;
            }
        }
        out
    }
}
struct Solution;
fn main() {}
