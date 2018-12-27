fn main() {
    let input = vec![
        vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
        vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
        vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
        vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    println!("{}", Solution::is_valid_sudoku(input));
}
struct Solution {}

impl Solution {
    pub fn check(nine: &Vec<char>) -> bool {
        let mut digits: [i32; 10] = [0; 10];
        for c in nine.iter() {
            let v = *c;
            if v == '.' {
                continue;
            }
            let digit = v as i32 - '0' as i32;
            if digits[digit as usize] == digit {
                return false;
            } else {
                digits[digit as usize] = digit;
            }
        }
        return true;
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            // Check lines
            let line = &board[i];
            if !Self::check(line) {
                return false;
            }
            let row = vec![
                board[0][i],
                board[1][i],
                board[2][i],
                board[3][i],
                board[4][i],
                board[5][i],
                board[6][i],
                board[7][i],
                board[8][i],
            ];
            // Check rows
            if !Self::check(&row) {
                return false;
            }
        }
        // Check little grids
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let lite_grid = vec![
                    board[i + 0][j + 0],
                    board[i + 1][j + 0],
                    board[i + 2][j + 0],
                    board[i + 0][j + 1],
                    board[i + 1][j + 1],
                    board[i + 2][j + 1],
                    board[i + 0][j + 2],
                    board[i + 1][j + 2],
                    board[i + 2][j + 2],
                ];
                if !Self::check(&lite_grid) {
                    return false;
                }
            }
        }
        true
    }
}
