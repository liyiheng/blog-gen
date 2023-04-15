use leetcode::TreeNode;
struct Solution;
fn main() {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::construct(&nums)
    }
    fn construct(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mut max_i = 0;
        let mut max_v = -1;
        for (i, &v) in nums.iter().enumerate() {
            if v > max_v {
                max_v = v;
                max_i = i;
            }
        }
        let left = Solution::construct(&nums[0..max_i]);
        let right = Solution::construct(&nums[max_i + 1..]);
        let n = TreeNode {
            val: max_v,
            left,
            right,
        };
        Some(Rc::new(RefCell::new(n)))
    }
}
