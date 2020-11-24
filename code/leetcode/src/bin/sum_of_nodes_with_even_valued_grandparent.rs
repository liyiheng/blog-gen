#![allow(dead_code)]
struct Solution;
fn main() {}
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
    fn son_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            node.borrow()
                .right
                .as_ref()
                .map(|v| v.borrow().val)
                .unwrap_or_default()
                + node
                    .borrow()
                    .left
                    .as_ref()
                    .map(|v| v.borrow().val)
                    .unwrap_or_default()
        } else {
            0
        }
    }
    fn grandson_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let r = node.borrow().right.clone();
            let l = node.borrow().left.clone();
            return Solution::son_sum(r) + Solution::son_sum(l);
        }
        0
    }
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            let sum = if node.borrow().val % 2 == 0 {
                Solution::grandson_sum(Some(node))
            } else {
                0
            };

            return sum
                + Solution::sum_even_grandparent(left)
                + Solution::sum_even_grandparent(right);
        }
        0
    }
}
