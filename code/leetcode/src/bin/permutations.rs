impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let cur_result = vec![vec![]];
        return Solution::insert(cur_result, nums);
    }

    // Input nums: [1, 2, 3]
    // cur_result: [ [] ]
    //
    // Round 1:
    //      nums: [1, 2], 3 popped
    //      cur_result: [ [3] ]
    // Round 2:
    //      nums: [1], 2 popped
    //      cur_result: [[2, 3], [3, 2]]
    // Round 3:
    //      nums: [], 1 popped
    //      cur_result: [[1, 2, 3], [2, 1, 3], [2, 3, 1], [1, 3, 2], [3, 1, 2], [3, 2, 1]]
    fn insert(cur_result: Vec<Vec<i32>>, mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return cur_result;
        }
        let mut result = vec![];
        let v = nums.pop().unwrap();
        for base in cur_result {
            for i in 0..=base.len() {
                let mut x = base.clone();
                x.insert(i, v);
                result.push(x);
            }
        }
        return Solution::insert(result, nums);
    }
}
struct Solution;

fn main() {
    let a = vec![1, 2, 3];
    let r = Solution::permute(a);
    for x in r {
        println!("{:?}", x);
    }
}
