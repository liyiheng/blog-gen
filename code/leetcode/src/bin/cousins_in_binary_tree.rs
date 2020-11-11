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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let (p1, d1) = Solution::find(root.clone(), x, 0);
        let (p2, d2) = Solution::find(root.clone(), y, 0);
        p1 != p2 && d1 == d2
    }
    fn find(root: Option<Rc<RefCell<TreeNode>>>, x: i32, cur: i32) -> (i32, i32) {
        let right = root.as_ref().map(|r| r.borrow().right.clone());
        let left = root.as_ref().map(|r| r.borrow().left.clone());
        if let Some(Some(node)) = right {
            let right_v = node.borrow().val;
            if right_v == x {
                return (root.unwrap().borrow().val, cur);
            }
            let (parent, depth) = Solution::find(Some(node), x, cur + 1);
            if depth > 0 {
                return (parent, depth);
            }
        }
        if let Some(Some(node)) = left {
            let v = node.borrow().val;
            if v == x {
                return (root.unwrap().borrow().val, cur);
            }
            let (parent, depth) = Solution::find(Some(node), x, cur + 1);
            if depth > 0 {
                return (parent, depth);
            }
        }
        (-1, -1)
    }
}

struct Solution;
fn main() {
    Solution::is_cousins(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    right: None,
                    left: None,
                }))),
                left: None,
            }))),
        }))),
        4,
        5,
    );
}
