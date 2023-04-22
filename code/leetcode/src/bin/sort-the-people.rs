struct Solution;
fn main() {}
impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut a: Vec<(String, i32)> = names.into_iter().zip(heights.into_iter()).collect();
        a.sort_by_key(|v| -v.1);
        a.into_iter().map(|v| v.0).collect()
    }
}
