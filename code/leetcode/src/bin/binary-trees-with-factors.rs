struct Solution;
fn main() {
    dbg!(Solution::num_factored_binary_trees(vec![2, 4]));
    dbg!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
}
const MOD: i64 = 1000000000 + 7;
impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::<i32, i64>::new();
        arr.sort_unstable();
        let mut ans = 0;
        for (i, &n) in arr.iter().enumerate() {
            let mut count = 1;
            for &m in arr[..i].iter() {
                if n % m != 0 {
                    continue;
                }
                let anoter = n / m;
                if anoter < m {
                    break;
                }
                if let Some(v1) = counter.get(&anoter) {
                    let c = if m == anoter {
                        v1 * v1 % MOD
                    } else {
                        let v2 = counter.get(&m).unwrap();
                        (v1 * v2 % MOD) * 2 % MOD
                    };
                    count += c % MOD;
                    count %= MOD;
                }
            }
            ans += count;
            ans %= MOD;
            counter.insert(n, count);
        }
        (ans % MOD) as i32
    }
}
