impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix.len();
        let center = size / 2;
        for i in 0..center {
            for j in i..size - 1 - i {
                let v1 = matrix[i][j];
                let (x, y) = Solution::trans(size, i, j);
                let v2 = matrix[x][y];
                matrix[x][y] = v1;
                let (x, y) = Solution::trans(size, x, y);
                let v3 = matrix[x][y];
                matrix[x][y] = v2;
                let (x, y) = Solution::trans(size, x, y);
                let v4 = matrix[x][y];
                matrix[x][y] = v3;
                matrix[i][j] = v4;
            }
        }
    }
    fn trans(size: usize, x: usize, y: usize) -> (usize, usize) {
        (y, size - 1 - x)
    }
}
struct Solution;
pub fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    for line in matrix {
        println!("{:?}", line);
    }
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    for line in matrix {
        println!("{:?}", line);
    }
}
