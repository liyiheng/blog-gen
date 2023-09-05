struct Solution;
fn main() {
    dbg!(Solution::min_trio_degree(
        6,
        [
            [6, 5],
            [4, 3],
            [5, 1],
            [1, 4],
            [2, 3],
            [4, 5],
            [2, 6],
            [1, 3]
        ]
        .map(|v| v.to_vec())
        .to_vec(),
    ));
}
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let edge_set: HashSet<(i32, i32)> = edges
            .iter()
            .map(|p| (p[0].min(p[1]), p[0].max(p[1])))
            .collect();
        let mut degrees = vec![0; n as usize + 1];
        let mut l_to_b = HashMap::<i32, HashSet<i32>>::new();
        for i in 1..=n {
            l_to_b.insert(i, HashSet::new());
        }
        for pair in edges {
            let a = pair[0];
            let b = pair[1];
            l_to_b.entry(a.min(b)).or_default().insert(a.max(b));
            degrees[a as usize] += 1;
            degrees[b as usize] += 1;
        }
        let get_degree = |v: i32| -> i32 { degrees[v as usize] };
        let gen_tragets = |v: i32| l_to_b.get(&v).unwrap().iter();
        let mut ans = i32::MAX;
        for i in 1..=n {
            let degree_i = get_degree(i);
            for &j in gen_tragets(i) {
                let degree_j = get_degree(j);
                for &k in gen_tragets(j) {
                    let pair = (i.min(k), i.max(k));
                    println!("{i} {j} {k}",);
                    if edge_set.contains(&pair) {
                        let degree_k = get_degree(k);
                        println!("trio {i} {j} {k}: {degree_i} {degree_j} {degree_k}",);
                        let degree = degree_i + degree_j + degree_k - 6;
                        ans = ans.min(degree);
                    }
                }
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
