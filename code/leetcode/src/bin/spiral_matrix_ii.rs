impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n <= 0 {
            return vec![];
        }
        let l = n as usize;
        let mut result = Vec::with_capacity(l);
        unsafe {
            for _ in 0..l {
                let mut v = Vec::with_capacity(l);
                v.set_len(l);
                result.push(v);
            }
        }

        let (mut x, mut y) = (0, 0);
        let (mut top, mut start) = (0, 0);

        let mut bottom = n - 1;
        let mut end = n - 1;

        let (right, down, left, up) = (1, 2, 3, 4);
        let mut direct = right;
        let mut cur = 1;
        loop {
            if start > end || top > bottom {
                break;
            }
            result[x as usize][y as usize] = cur;
            cur += 1;
            if direct == right {
                if y < end {
                    y += 1;
                } else {
                    direct = down;
                    top += 1;
                    x += 1;
                }
                continue;
            }
            if direct == down {
                if x < bottom {
                    x += 1;
                } else {
                    direct = left;
                    end -= 1;
                    y -= 1;
                }
                continue;
            }
            if direct == left {
                if y > start {
                    y -= 1;
                } else {
                    direct = up;
                    bottom -= 1;
                    x -= 1;
                }
                continue;
            }
            if direct == up {
                if x > top {
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
fn main() {}
