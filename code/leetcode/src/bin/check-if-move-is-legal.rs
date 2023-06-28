struct Solution;
fn main() {}
impl Solution {
    pub fn check_move(mut board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        struct V(i32, i32);
        impl V {
            fn valid(&self) -> bool {
                self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
            }
        }
        let directions = [
            V(-1, -1),
            V(0, -1),
            V(1, -1),
            V(-1, 0),
            V(1, 0),
            V(-1, 1),
            V(0, 1),
            V(1, 1),
        ];
        let border: Vec<(V, V)> = directions
            .iter()
            .map(|&V(dr, dc)| (V(dr, dc), V(dr + r_move, dc + c_move)))
            .filter(|(_, p)| p.0 >= 0 && p.1 >= 0 && p.0 < 8 && p.1 < 8)
            .filter(|(_, p)| {
                let r = p.0 as usize;
                let c = p.1 as usize;
                let v = board[r][c];
                v != color && v != '.'
            })
            .collect();
        if border.is_empty() {
            return false;
        }
        border.into_iter().any(|(d, mut p)| loop {
            let next = V(d.0 + p.0, d.1 + p.1);
            p = next;
            if !p.valid() {
                return false;
            }
            let v = board[p.0 as usize][p.1 as usize];
            if v == '.' {
                return false;
            }
            if v == color {
                return true;
            }
        })
    }
}
