impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as f64;
        let mut x1 = x;
        let mut x2 = (x / x1 + x1) / 2.0;
        while (x1 - x2).abs() > 0.1 {
            x1 = x2;
            x2 = (x / x1 + x1) / 2.0;
        }
        x2 as i32
    }
}

struct Solution;
fn main() {
    for i in 1..1000 {
        let v = (i as f32).sqrt() as i32;
        assert_eq!(v, Solution::my_sqrt(i));
        if i == 666 {
            let i = 2147395599;
            let v = (i as f64).sqrt() as i32;
            println!("{}", (i as f32).sqrt());
            assert_eq!(v, Solution::my_sqrt(i));
        }
    }
}
