struct Solution;

fn main() {}
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        use std::collections::HashSet;
        let j: HashSet<u8> = jewels.bytes().collect();
        stones.bytes().filter(|b| j.contains(b)).count() as i32
    }
}
