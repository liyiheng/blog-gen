struct Solution;
pub fn main() {
    Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]);
    Solution::garden_no_adj(20, vec![]);
}
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut all_paths = vec![vec![]; n as usize + 1];
        for pair in paths.iter() {
            let a = pair[0];
            let b = pair[1];
            all_paths[a as usize].push(b);
            all_paths[b as usize].push(a);
        }
        let mut ans = vec![0; n as usize];
        for i in 1..=n {
            let neighbor = &mut all_paths[i as usize];
            let mut kinds = vec![false; 5];
            kinds[ans[i as usize - 1] as usize] = true;
            for &v in neighbor.iter() {
                kinds[ans[v as usize - 1] as usize] = true;
            }
            for k in 1..5 {
                if !kinds[k] {
                    kinds[k] = true;
                    ans[i as usize - 1] = k as i32;
                    break;
                }
            }
        }
        dbg!(ans)
    }
}
