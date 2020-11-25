#![allow(dead_code)]
struct Solution;
fn main() {}
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let size = grid.len();
        let mut top = vec![0; size];
        let mut left = vec![0; size];
        for i in 0..size {
            for j in 0..size {
                left[i] = left[i].max(grid[i][j]);
                top[j] = top[j].max(grid[i][j]);
            }
        }
        let mut increase = 0;
        for i in 0..size {
            for j in 0..size {
                let delta = top[j].min(left[i]) - grid[i][j];
                increase += delta;
            }
        }
        increase
    }
}
