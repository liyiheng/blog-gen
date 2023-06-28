struct Solution;
fn main() {
    dbg!(Solution::row_and_maximum_ones(vec![]));
}
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        mat.into_iter()
            .map(|row| row.into_iter().filter(|&v| v == 1).count() as i32)
            .enumerate()
            .reduce(|(maxi, maxc), (i, c)| {
                if c > maxc {
                    return (i, c);
                }
                (maxi, maxc)
            })
            .map(|(i, c)| vec![i as i32, c])
            .unwrap()
    }
}
