struct Solution;
fn main() {}

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let total: i32 = skill.iter().sum();
        if total * 2 % skill.len() as i32 != 0 {
            return -1;
        }
        let two_sum = total * 2 / skill.len() as i32;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for v in skill {
            *counter.entry(v).or_default() += 1;
        }
        let mut ans: i64 = 0;
        while counter.len() > 0 {
            let (&sp, &count) = counter.iter().next().unwrap();
            let target = two_sum - sp;
            let n = counter.get(&target).cloned().unwrap_or_default();
            if n != count {
                return -1;
            }
            counter.remove(&sp);
            counter.remove(&target);
            if sp == target {
                if n % 2 != 0 {
                    return -1;
                }
                ans += sp as i64 * target as i64 * (n / 2) as i64;
            } else {
                ans += sp as i64 * target as i64 * n as i64;
            }
        }
        ans
    }
}
