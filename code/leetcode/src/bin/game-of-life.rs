fn main() {}
struct Solution;
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let width = board[0].len();
        let height = board.len();
        let mut next = vec![vec![0; width]; height];
        for i in 0..width {
            for j in 0..height {
                let mut cnt = 0;
                if i > 0 {
                    cnt += board[j][i - 1];
                    cnt += board.get(j + 1).map(|v| v[i - 1]).unwrap_or_default();
                }
                if j > 0 {
                    cnt += board[j - 1][i];
                    cnt += board[j - 1].get(i + 1).cloned().unwrap_or_default();
                }
                if j > 0 && i > 0 {
                    cnt += board[j - 1][i - 1];
                }
                cnt += board
                    .get(j + 1)
                    .map(|v| v.get(i))
                    .flatten()
                    .cloned()
                    .unwrap_or_default();
                cnt += board
                    .get(j + 1)
                    .map(|v| v.get(i + 1))
                    .flatten()
                    .cloned()
                    .unwrap_or_default();
                cnt += board
                    .get(j)
                    .map(|v| v.get(i + 1))
                    .flatten()
                    .cloned()
                    .unwrap_or_default();
                match cnt {
                    0 | 1 => next[j][i] = 0,
                    2 => next[j][i] = board[j][i],
                    3 => next[j][i] = 1,
                    4 | 5 | 6 | 7 | 8 => next[j][i] = 0,
                    _ => unreachable!(),
                }
            }
        }
        for (i, line) in next.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                board[i][j] = v;
            }
        }
    }
}
