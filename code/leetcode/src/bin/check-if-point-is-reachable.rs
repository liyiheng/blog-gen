struct Solution;
fn main() {
    dbg!(Solution::is_reachable(6, 9));
    dbg!(Solution::is_reachable(4, 7));
    dbg!(Solution::is_reachable(757172937, 869964136));
}

use std::collections::HashMap;
fn shrink(mut v: i32) -> i32 {
    while v % 2 == 0 {
        v /= 2;
    }
    v
}
fn dfs(mut x: i32, mut y: i32, mem: &mut HashMap<(i32, i32), bool>) -> bool {
    if x == 1 || y == 1 {
        return true;
    }
    if x == y {
        return false;
    }
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    if let Some(&b) = mem.get(&(x, y)) {
        return b;
    }
    mem.insert((x, y), false);
    let z = shrink(x + y);
    if dfs(x, z, mem) {
        mem.insert((x.min(z), x.max(z)), true);
        return true;
    }
    false
}

impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        let x = shrink(target_x);
        let y = shrink(target_y);
        let mut mem = HashMap::new();
        dfs(x, y, &mut mem)
    }
}
