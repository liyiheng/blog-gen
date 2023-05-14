struct Solution;
use leetcode::TreeNode;
fn main() {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = vec![];
        Solution::push_leaves(root, 0, &mut sums);
        sums.last().cloned().unwrap_or_default()
    }
    fn push_leaves(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, sums: &mut Vec<i32>) {
        while sums.len() < depth + 1 {
            sums.push(0);
        }
        if let Some(node) = root {
            if node.borrow().right.is_none() && node.borrow().left.is_none() {
                sums[depth] += node.borrow().val;
            } else {
                Solution::push_leaves(node.borrow().right.clone(), depth + 1, sums);
                Solution::push_leaves(node.borrow().left.clone(), depth + 1, sums);
            }
        }
    }
}
