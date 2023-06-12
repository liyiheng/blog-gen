struct Solution;
fn main() {
    Solution::reconstruct_matrix(0, 0, vec![]);
}
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let sum: i32 = colsum.iter().sum();
        if sum != upper + lower {
            return vec![];
        }
        let n = colsum.len();
        let mut ans = vec![vec![0; n], vec![0; n]];
        let mut diff_indexes = vec![];
        let mut cur_u = 0;
        let mut cur_l = 0;
        for (i, s) in colsum.into_iter().enumerate() {
            match s {
                0 => {
                    ans[0][i] = 0;
                    ans[1][i] = 0;
                }
                2 => {
                    ans[0][i] = 1;
                    ans[1][i] = 1;
                    cur_u += 1;
                    cur_l += 1;
                }
                _ => diff_indexes.push(i),
            }
        }
        if dfs(&mut ans, &diff_indexes, upper, lower, cur_u, cur_l) {
            ans
        } else {
            vec![]
        }
    }
}
fn dfs(ans: &mut [Vec<i32>], indexes: &[usize], upper: i32, lower: i32, cu: i32, cl: i32) -> bool {
    if indexes.is_empty() {
        return cu == upper && cl == lower;
    }
    if cu > upper || cl > lower {
        return false;
    }
    let i = indexes[0];
    ans[0][i] = 0;
    ans[1][i] = 1;
    if dfs(ans, &indexes[1..], upper, lower, cu, cl + 1) {
        return true;
    }
    ans[0][i] = 1;
    ans[1][i] = 0;
    dfs(ans, &indexes[1..], upper, lower, cu + 1, cl)
}
