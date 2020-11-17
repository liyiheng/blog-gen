impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let l = a.len();
        if l < 3 {
            return false;
        }
        let sum: i32 = a.iter().sum();
        let mut sum1 = 0;
        for start in 1..l - 1 {
            sum1 += a[start - 1];
            let left = sum - sum1;
            let mut sum2 = 0;
            for end in start..l - 1 {
                sum2 += a[end];
                println!("\t{},{}, sum1:{}, sum2:{}", start, end, sum1, sum2);
                let sum3: i32 = left - sum2;
                if sum1 == sum2 && sum2 == sum3 {
                    return true;
                }
            }
        }
        false
    }
}
struct Solution;
fn main() {
    assert!(!Solution::can_three_parts_equal_sum(vec![1, -1, 1, -1]));
    assert!(Solution::can_three_parts_equal_sum(vec![
        18, 12, -18, 18, -19, -1, 10, 10
    ]));

    assert!(Solution::can_three_parts_equal_sum(vec![
        0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
    ]));
}
