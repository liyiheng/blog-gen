struct Solution;
fn main() {}
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let sum = sum / 3;
        let mut i = 0;
        let mut j = 0;
        let mut sum1 = 0;
        for (idx, &v) in arr.iter().enumerate() {
            i = idx;
            sum1 += v;
            if sum1 == sum {
                break;
            }
        }
        sum1 = 0;
        for (ii, &v) in arr.iter().enumerate().rev() {
            j = ii;
            sum1 += v;
            if sum1 == sum {
                break;
            }
        }
        i < j
    }
}
