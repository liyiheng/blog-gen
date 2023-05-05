struct Solution;
fn main() {}
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let width = mat[0].len();
        let height = mat.len();
        let mut skip_x = vec![false; height];
        let mut skip_y = vec![false; width];
        let mut ans = 0;
        for x in 0..height {
            if skip_x[x] {
                continue;
            }
            for y in 0..width {
                if skip_y[y] {
                    continue;
                }
                let v = mat[x][y];
                if v == 0 {
                    continue;
                }
                let mut is_special = true;
                for xx in 0..height {
                    if xx == x {
                        continue;
                    }
                    if mat[xx][y] == 1 {
                        is_special = false;
                        break;
                    }
                }
                if !is_special {
                    continue;
                }
                for yy in 0..width {
                    if yy == y {
                        continue;
                    }
                    if mat[x][yy] == 1 {
                        is_special = false;
                        break;
                    }
                }
                if is_special {
                    skip_y[y] = true;
                    skip_x[x] = true;
                    ans += 1;
                }
            }
        }
        ans
    }
}
