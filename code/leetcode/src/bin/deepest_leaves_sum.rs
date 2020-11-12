// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut leaves = vec![];
        Solution::push_leaves(root, 0, &mut leaves);
        let mut depth = 0;
        let mut sum = 0;
        for (d, v) in leaves {
            if d == depth {
                sum += v;
            } else if d > depth {
                depth = d;
                sum = v;
            }
        }
        sum
    }
    fn push_leaves(root: Option<Rc<RefCell<TreeNode>>>, depth: u32, leaves: &mut Vec<(u32, i32)>) {
        if let Some(node) = root {
            if node.borrow().right.is_none() && node.borrow().left.is_none() {
                leaves.push((depth, node.borrow().val));
            } else {
                Solution::push_leaves(node.borrow().right.clone(), depth + 1, leaves);
                Solution::push_leaves(node.borrow().left.clone(), depth + 1, leaves);
            }
        }
    }
}

struct Solution;
fn main() {}
