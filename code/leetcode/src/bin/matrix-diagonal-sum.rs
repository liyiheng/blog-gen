fn main() {}
struct Solution;
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let l = mat.len();
        let mut ans = 0;
        for (i, line) in mat.iter().enumerate() {
            let a = line[i];
            let b = line[l - 1 - i];
            ans += a + b;
        }
        if l % 2 == 1 {
            ans -= mat[l / 2][l / 2];
        }
        ans
    }
}
