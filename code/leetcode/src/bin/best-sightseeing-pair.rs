struct Solution;
fn main() {}
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        // i<j,  values[i] +valuse[j] + i-j
        //       values[i]+i  + values[j]-j
        let first_station: Vec<i32> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| v + i as i32)
            .collect();
        let second_station: Vec<i32> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| v - i as i32)
            .collect();
        let mut max_i = vec![-1; values.len()];
        let mut max_j = vec![-1; values.len()];
        max_i[0] = first_station[0];
        max_j[values.len() - 1] = *second_station.last().unwrap();
        for i in 1..values.len() {
            max_i[i] = first_station[i].max(max_i[i - 1]);
        }
        for j in (0..(values.len() - 1)).rev() {
            max_j[j] = second_station[j].max(max_j[j + 1]);
        }
        let mut ans = 0;
        for i in 0..(values.len() - 1) {
            let vv = max_i[i] + max_j[i + 1];
            ans = ans.max(vv);
        }
        ans
    }
}
