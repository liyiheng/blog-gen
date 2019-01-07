impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        return 1162261467 % n == 0;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_power_of_three(81));
    println!("{}", Solution::is_power_of_three(27));
    println!("{}", Solution::is_power_of_three(15));
    println!("{}", Solution::is_power_of_three(18));
    let mut max: i32 = 1;
    loop {
        if let Some(v) = max.checked_mul(3) {
            max = v;
        } else {
            break;
        }
    }
    println!("Max:{}", max); // 1162261467
}
