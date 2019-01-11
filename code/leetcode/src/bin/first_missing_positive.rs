impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut max = 0;
        for i in nums {
            set.insert(i);
            if i > max {
                max = i;
            }
        }
        let mut i = 1;
        while i < max {
            if !set.contains(&i) {
                return i;
            }
            i += 1;
        }
        max + 1
    }
}

struct Solution {}
fn main() {
    println!("{}", Solution::first_missing_positive(vec![7, 8, 9]));
}
