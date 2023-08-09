struct Solution;
fn main() {
    dbg!(Solution::min_falling_path_sum(vec![]));
}
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 1 {
            return *matrix[0].iter().min().unwrap();
        }
        for i in 1..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut prev_v = matrix[i - 1][j];
                if j > 0 {
                    prev_v = prev_v.min(matrix[i - 1][j - 1]);
                }
                if j + 1 < matrix[0].len() {
                    prev_v = prev_v.min(matrix[i - 1][j + 1]);
                }
                matrix[i][j] += prev_v;
            }
        }
        *matrix.last().unwrap().iter().min().unwrap()
    }
}
