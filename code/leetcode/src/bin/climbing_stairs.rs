impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut prev_prev = 1;
        let mut prev = 2;
        for _ in 3..=n {
            let x = prev_prev + prev;
            prev_prev = prev;
            prev = x;
        }
        prev
    }
}

struct Solution;
fn main() {
    println!("{}", Solution::climb_stairs(1));
    println!("{}", Solution::climb_stairs(2));
    println!("{}", Solution::climb_stairs(3));
}
