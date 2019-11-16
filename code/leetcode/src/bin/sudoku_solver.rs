impl Solution {
    fn step(board: &mut Vec<Vec<char>>, index: usize) -> bool {
        if index == 81 {
            return true;
        }
        let mut next_index = 82;
        let mut x = 0;
        let mut y = 0;
        for i in index..81 {
            x = i % 9;
            y = i / 9;
            if board[x][y] == '.' {
                next_index = i + 1;
                break;
            }
        }
        if next_index == 82 {
            return true;
        }

        let candidates = Self::get_candidates(board, x, y);
        if candidates.len() == 0 {
            return false;
        }
        for &c in candidates.iter() {
            board[x][y] = c;
            let ok = Self::step(board, next_index);
            if ok {
                return ok;
            }
        }
        board[x][y] = '.';
        return false;
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::step(board, 0);
    }

    fn get_candidates(board: &mut Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
        let mut candidates = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for &c in board[x].iter() {
            if c == '.' {
                continue;
            }
            let v = c as u8 - '0' as u8;
            candidates[v as usize - 1] = '.';
        }
        for line in board.iter() {
            let c = line[y];
            if c == '.' {
                continue;
            }
            let v = c as u8 - '0' as u8;
            candidates[v as usize - 1] = '.';
        }
        let min_x = x / 3 * 3;
        let min_y = y / 3 * 3;
        for inner_x in min_x..min_x + 3 {
            for inner_y in min_y..min_y + 3 {
                let c = board[inner_x][inner_y];
                if c == '.' {
                    continue;
                }
                let v = c as u8 - '0' as u8;
                candidates[v as usize - 1] = '.';
            }
        }
        candidates.retain(|c| *c != '.');
        return candidates;
    }
}

struct Solution;
fn main() {
    let mut a = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut a);
    for aa in a.iter() {
        println!("{:?}", aa);
    }
}
