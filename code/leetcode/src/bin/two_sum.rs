fn main() {
    let r = Solution::two_sum(vec![], 0);
    println!("{:?}", r);
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::default();
        let numbers = 0..;
        for i in numbers.take(nums.len()) {
            let v = nums[i];
            let v2 = target - v;
            let i2 = num_map.get(&v2);
            if let Some(t) = i2 {
                if *t != i as i32 {
                    return vec![i as i32, *t];
                }
            }
            num_map.insert(v, i as i32);
        }
        return vec![];
    }

    // Edition 2015, without Non-lexical lifetime
    pub fn two_sum_2015(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::default();
        let numbers = 0..;
        for i in numbers.take(nums.len()) {
            let v = nums[i];
            let v2 = target - v;
            let i2 = {
                let tmp: Option<&i32> = num_map.get(&v2);
                if tmp.is_some() {
                    Some(*tmp.unwrap())
                } else {
                    None
                }
            };
            if let Some(t) = i2 {
                if t != i as i32 {
                    return vec![i as i32, t];
                }
            }
            num_map.insert(v, i as i32);
        }
        return vec![];
    }
}
