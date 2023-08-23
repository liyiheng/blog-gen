fn main() {
    dbg!(Solution::count_servers(vec![vec![1, 0], vec![1, 1]]));
    dbg!(Solution::count_servers(vec![
        vec![1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ]));
}
struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut multi_peer_lines = HashSet::<usize>::new();
        let mut multi_peer_col = HashSet::<usize>::new();
        let mut results = vec![vec![false; grid[0].len()]; grid.len()];
        for (i, line) in grid.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                if v == 0 {
                    continue;
                }
                if multi_peer_col.contains(&j) || multi_peer_lines.contains(&i) {
                    results[i][j] = true;
                    multi_peer_lines.insert(i);
                    multi_peer_col.insert(j);
                    continue;
                }
                if results[i][j] {
                    continue;
                }
                // to right
                let mut another_exists = false;
                for (k, &vv) in (line[j + 1..]).iter().enumerate() {
                    if vv == 1 {
                        another_exists = true;
                        results[i][j + k + 1] = true;
                        multi_peer_lines.insert(i);
                        multi_peer_col.insert(j + k + 1);
                    }
                }
                // to down
                for k in i + 1..grid.len() {
                    if grid[k][j] == 1 {
                        another_exists = true;
                        results[k][j] = true;
                        multi_peer_lines.insert(k);
                        multi_peer_col.insert(j);
                    }
                }
                if another_exists {
                    results[i][j] = true;
                    multi_peer_lines.insert(i);
                    multi_peer_col.insert(j);
                }
            }
        }
        for line in results.iter() {
            println!("{line:?}");
        }
        results
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|&v| v)
            .count() as i32
    }
}
