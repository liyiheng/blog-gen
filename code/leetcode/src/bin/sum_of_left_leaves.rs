#![allow(dead_code)]
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.as_ref().unwrap().borrow();
        let sum_right = root
            .right
            .as_ref()
            .map(|r| Self::sum_of_left_leaves(Some(Rc::clone(r))))
            .unwrap_or(0);
        let sum_left = if let Some(left) = root.left.as_ref() {
            if left.borrow().left.is_none() {
                if left.borrow().right.is_none() {
                    left.borrow().val
                } else {
                    Self::sum_of_left_leaves(Some(Rc::clone(left.borrow().right.as_ref().unwrap())))
                }
            } else {
                Self::sum_of_left_leaves(Some(Rc::clone(left)))
            }
        } else {
            0
        };
        sum_left + sum_right
    }
}
struct Solution {}
fn main() {}
