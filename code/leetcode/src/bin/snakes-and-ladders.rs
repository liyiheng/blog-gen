struct Solution;
fn main() {
    let a = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    dbg!(Solution::snakes_and_ladders(a));
    let a = vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, 1]];
    dbg!(Solution::snakes_and_ladders(a));
}
fn bfs(board: &mut Board) -> i32 {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    let last = board.width().pow(2);
    let mut q = std::collections::VecDeque::new();
    // (current postion, current steps)
    q.push_back((1usize, 0usize));
    while !q.is_empty() {
        let (cur, steps) = q.pop_front().unwrap();
        if cur == last {
            return steps as i32;
        }
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        for i in 1..=6 {
            if cur + i > last {
                break;
            }
            let dest = cur + i;
            let cur = board.advance(dest);
            if cur == last {
                return steps as i32 + 1;
            }
            if cur > last {
                break;
            }
            q.push_back((cur, steps + 1));
        }
    }
    -1
}
#[inline]
fn position(width: usize, cur: usize) -> (usize, usize) {
    let i = cur - 1;
    let y = i / width;
    let mut x = i % width;
    if y % 2 == 1 {
        x = width - 1 - x;
    }
    (x, width - 1 - y)
}
struct Board(Vec<Vec<i32>>);
impl Board {
    fn width(&self) -> usize {
        self.0.len()
    }
    fn advance(&mut self, dest: usize) -> usize {
        let (x, y) = position(self.width(), dest);
        let val = self.0[y][x];
        if val != -1 {
            val as usize
        } else {
            dest
        }
    }
}
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut b = Board(board);
        bfs(&mut b)
    }
}
