struct Solution;
fn main() {
    let mut x = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    //    dbg!(x.get_kth_ancestor(3, 1));
    dbg!(x.get_kth_ancestor(5, 2));
    //   dbg!(x.get_kth_ancestor(6, 3));
}
struct TreeAncestor {
    parents: Vec<Vec<i32>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut parents = vec![vec![]; n as usize];
        for (i, v) in parent.into_iter().enumerate() {
            let mut tmp = vec![-1; 16];
            tmp[0] = v;
            parents[i] = tmp;
        }
        for lvl in 1..16 {
            for i in 0..n as usize {
                let half = parents[i][lvl - 1];
                if half != -1 {
                    parents[i][lvl] = parents[half as usize][lvl - 1];
                }
            }
        }
        TreeAncestor { parents }
    }
    fn get_kth_ancestor(&mut self, node: i32, k: i32) -> i32 {
        match k {
            0 => return node,
            1 => return self.parents[node as usize][0],
            2 => return self.parents[node as usize][1],
            4 => return self.parents[node as usize][2],
            8 => return self.parents[node as usize][3],
            16 => return self.parents[node as usize][4],
            32 => return self.parents[node as usize][5],
            64 => return self.parents[node as usize][6],
            _ => {}
        }
        for i in (0..16).rev() {
            let threshold = 1 << i;
            if k >= threshold {
                let p = self.parents[node as usize][i];
                if p == -1 {
                    return -1;
                }
                return self.get_kth_ancestor(p, k - threshold);
            }
        }
        -1
    }
}
