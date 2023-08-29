struct Solution;
fn main() {
    //    dbg!(Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9));
    //    dbg!(Solution::minimum_jumps(
    //        vec![8, 3, 16, 6, 12, 20],
    //        15,
    //        13,
    //        11
    //    ));
    //    dbg!(Solution::minimum_jumps(vec![18, 13, 3, 9, 8, 14], 3, 8, 6));
    dbg!(Solution::minimum_jumps(
        vec![
            162, 118, 178, 152, 167, 100, 40, 74, 199, 186, 26, 73, 200, 127, 30, 124, 193, 84,
            184, 36, 103, 149, 153, 9, 54, 154, 133, 95, 45, 198, 79, 157, 64, 122, 59, 71, 48,
            177, 82, 35, 14, 176, 16, 108, 111, 6, 168, 31, 134, 164, 136, 72, 98
        ],
        29,
        98,
        80
    ));
}
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let limit = if a == b {
            x
        } else {
            // cheat: max(max(forbidden)+a,x)+b
            let mx = *forbidden.iter().max_by(|n1, n2| n1.cmp(n2)).unwrap();
            (mx + a).max(x) + b
        };

        let forbidden: HashSet<i32> = forbidden.into_iter().collect();
        let mut memory: HashSet<i32> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((true, a, 1));
        while !queue.is_empty() {
            let (forward, pos, steps) = queue.pop_front().unwrap();
            if pos > limit {
                continue;
            }
            if memory.contains(&pos) {
                continue;
            }
            if forward {
                memory.insert(pos);
            }
            if forbidden.contains(&pos) {
                continue;
            }
            if pos == x {
                return steps;
            }
            let mut push_back = |np, f| {
                if np > 0 && !forbidden.contains(&np) && !memory.contains(&np) {
                    queue.push_back((f, np, steps + 1));
                }
            };
            if pos < i32::MAX - a {
                push_back(pos + a, true);
            }
            if forward {
                push_back(pos - b, false);
            }
        }
        -1
    }
}
