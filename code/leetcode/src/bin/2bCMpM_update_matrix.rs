struct Solution;
fn main() {
    dbg!(Solution::update_matrix(vec![vec![
        1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1
    ]]));
}

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![i32::MAX; mat[0].len()]; mat.len()];
        let mut q = vec![];
        for (i, line) in mat.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                if v == 0 {
                    ans[i][j] = 0;
                    q.push((i, j));
                }
            }
        }
        let mut cur = 0;
        while !q.is_empty() {
            println!("{}", q.len());
            let mut next_q = vec![];
            cur += 1;
            for &(i, j) in q.iter() {
                if i > 0 && ans[i - 1][j] > cur {
                    ans[i - 1][j] = cur;
                    next_q.push((i - 1, j));
                }
                if i + 1 < ans.len() && ans[i + 1][j] > cur {
                    ans[i + 1][j] = cur;
                    next_q.push((i + 1, j));
                }
                if j + 1 < ans[0].len() && ans[i][j + 1] > cur {
                    ans[i][j + 1] = cur;
                    next_q.push((i, j + 1));
                }
                if j > 0 && ans[i][j - 1] > cur {
                    ans[i][j - 1] = cur;
                    next_q.push((i, j - 1));
                }
            }
            q = next_q;
        }
        ans
    }
}
