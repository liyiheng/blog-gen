struct Solution;
fn main() {}
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut counter: HashMap<String, i32> = HashMap::new();
        for line in matrix {
            let starts_with_zero = line[0] == 0;
            let feature: String = line
                .into_iter()
                .map(|mut v| {
                    if !starts_with_zero {
                        v = v ^ 1;
                    }
                    (b'0' + v as u8) as char
                })
                .collect();
            *counter.entry(feature).or_default() += 1;
        }
        counter.into_values().reduce(|a, b| a.max(b)).unwrap()
    }
}
