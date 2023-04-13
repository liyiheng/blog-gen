impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut matrix = vec![vec![]; num_rows];
        let mut down = true;
        let mut x = 0;
        let mut y = 0;
        for &b in s.as_bytes().iter() {
            while matrix[y].len() < x + 1 {
                matrix[y].push(None);
            }
            matrix[y][x] = Some(b);
            if down {
                if y + 1 == num_rows {
                    down = false;
                    y -= 1;
                    x += 1;
                } else {
                    y += 1;
                }
            } else {
                // up
                if y == 0 {
                    down = true;
                    y += 1;
                } else {
                    y -= 1;
                    x += 1;
                }
            }
        }
        let data = matrix
            .iter()
            .flat_map(|line| line.iter())
            .filter_map(|o| *o)
            .collect();
        String::from_utf8(data).unwrap()
    }
}
struct Solution;
fn main() {}
