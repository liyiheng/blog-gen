impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let (mut x, mut y) = (0, 0);
        let (mut top, mut start) = (0, 0);

        let mut bottom = matrix.len() as i32 - 1;
        let mut end = matrix.last().map(|v| v.len()).unwrap_or(0) as i32 - 1;

        let (right, down, left, up) = (1, 2, 3, 4);
        let mut direct = right;
        loop {
            if start > end || top > bottom {
                break;
            }
            let v = matrix[x][y];
            result.push(v);
            if direct == right {
                if y < end as usize {
                    y += 1;
                } else {
                    direct = down;
                    top += 1;
                    x += 1;
                }
                continue;
            }
            if direct == down {
                if x < bottom as usize {
                    x += 1;
                } else {
                    direct = left;
                    end -= 1;
                    y -= 1;
                }
                continue;
            }
            if direct == left {
                if y > start as usize {
                    y -= 1;
                } else {
                    direct = up;
                    bottom -= 1;
                    x -= 1;
                }
                continue;
            }
            if direct == up {
                if x > top as usize {
                    x -= 1;
                } else {
                    direct = right;
                    start += 1;
                    y += 1;
                }
                continue;
            }
            break;
        }
        result
    }
}
struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!("{:?}", Solution::spiral_order(vec![vec![1, 2], vec![3, 4]]));
}
