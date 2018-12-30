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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn is_leaf(n: Rc<RefCell<TreeNode>>) -> bool {
        n.borrow().left.is_none() && n.borrow().right.is_none()
    }
    fn add_sum(n: Rc<RefCell<TreeNode>>, sum: i32, current: i32) -> bool {
        let v = n.borrow().val;
        let now = current + v;
        if sum == now && Solution::is_leaf(n.clone()) {
            return true;
        }
        match Rc::clone(&n).borrow().left {
            Some(ref node) => {
                if Solution::add_sum(Rc::clone(node), sum, now) {
                    return true;
                }
            }
            None => {}
        };
        if let Some(ref node) = n.clone().borrow().right {
            return Solution::add_sum(Rc::clone(node), sum, now);
        }
        false
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let root = root.unwrap();
        Solution::add_sum(root, sum, 0)
    }
}

fn main() {
    println!(
        "Time Submitted\t\tStatus\tRuntime\tLanguage
2 minutes age\t\tAccepted\t4 ms\trust
7 minutes ago\t\tAccepted\t8 ms\tgolang
11 hours ago\t\tAccepted\t8 ms\tgolang"
    )
}
