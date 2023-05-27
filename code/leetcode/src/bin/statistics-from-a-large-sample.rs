struct Solution;
fn main() {}

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut ans = vec![0.01; 5];
        ans[0] = count.iter().enumerate().find(|&(_, &v)| v > 0).unwrap().0 as f64;
        ans[1] = count
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &v)| v > 0)
            .unwrap()
            .0 as f64;
        let sum: f64 = count
            .iter()
            .enumerate()
            .map(|(i, &v)| i * v as usize)
            .sum::<usize>() as f64;
        let total = count.iter().map(|&v| v as i64).sum::<i64>();
        ans[2] = sum / total as f64;
        ans[4] = count
            .iter()
            .enumerate()
            .reduce(|(i1, v1), (i2, v2)| if v1 > v2 { (i1, v1) } else { (i2, v2) })
            .map(|v| v.0)
            .unwrap() as f64;
        let target = total / 2;
        let mut cur = 0;
        for (i, &v) in count.iter().enumerate() {
            let v = v as i64;
            if cur <= target && cur + v > target {
                ans[3] = i as f64;
                break;
            }
            cur += v;
        }
        if total % 2 == 0 {
            let target = target - 1;
            let mut cur = 0;
            for (i, &v) in count.iter().enumerate() {
                let v = v as i64;
                if cur <= target && cur + v > target {
                    ans[3] += i as f64;
                    ans[3] *= 0.5;
                    break;
                }
                cur += v;
            }
        }
        ans
    }
}
