impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashSet::with_capacity(nums.len() / 2 + 1);
        for n in nums.iter() {
            if map.contains(n) {
                map.remove(n);
            } else {
                map.insert(*n);
            }
        }
        map.into_iter().next().unwrap()
    }
}
struct Solution;
fn main() {
    println!(
        "{}",
        Solution::single_number(vec![1, 2, 3, 4, 5, 4, 3, 2, 1])
    );
    println!("{}", Solution::single_number(vec![2, 2, 1]));
    println!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}
