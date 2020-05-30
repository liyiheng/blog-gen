impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut total = 0;
        let (mut i, mut j) = (0, height.len() - 1);
        let (mut max_a, mut max_b) = (0, 0);
        while i < j {
            if height[i] < height[j] {
                max_a = max_a.max(height[i]);
                total += max_a - height[i];
                i += 1;
            } else {
                max_b = max_b.max(height[j]);
                total += max_b - height[j];
                j -= 1;
            }
        }
        total
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    );
    println!("{}", Solution::trap(vec![5, 4, 1, 2]));
    println!("{}", Solution::trap(vec![5, 2, 1, 2, 1, 5]));
}
