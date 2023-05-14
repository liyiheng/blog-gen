struct Solution;

pub fn main() {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let remaind: Vec<i32> = gas
            .into_iter()
            .zip(cost.into_iter())
            .map(|(a, b)| a - b)
            .collect();
        if remaind.iter().sum::<i32>() < 0 {
            return -1;
        }
        let mut ans = 0;
        let mut cur = 0;
        for (i, v) in remaind.into_iter().enumerate() {
            cur += v;
            if cur < 0 {
                ans = i + 1;
                cur = 0;
            }
        }
        ans as i32
    }
}
