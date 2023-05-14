#![allow(dead_code)]
struct Solution;
fn main() {
    println!(
        "{}",
        Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]])
    );

    println!(
        "{}",
        Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]])
    );
}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut neg_cnt = 0;
        let mut v = matrix[0][0];
        let n = matrix.len();
        let mut sum = 0i64;
        for i in 0..n {
            for j in 0..n {
                let cur_v = matrix[i][j];
                if cur_v < 0 {
                    neg_cnt += 1;
                }
                if cur_v.abs() < v.abs() {
                    v = cur_v;
                }
                sum += cur_v.abs() as i64;
            }
        }
        if neg_cnt % 2 == 0 {
            sum
        } else {
            sum - v.abs() as i64 * 2
        }
    }
}
