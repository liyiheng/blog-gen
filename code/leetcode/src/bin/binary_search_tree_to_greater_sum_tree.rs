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
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_to_gst2(root, 0)
    }
    fn left_leave(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        while let Some(node) = root {
            if node.borrow().left.is_none() {
                println!("left_leave {}", node.borrow().val);
                return node.borrow().val;
            } else {
                root = node.borrow().left.clone();
            }
        }
        0
    }

    fn bst_to_gst2(
        root: Option<Rc<RefCell<TreeNode>>>,
        parent: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_node) = root {
            let right = root_node.borrow().right.clone();
            let left = root_node.borrow().left.clone();
            {
                let mut node = root_node.borrow_mut();
                node.right = Solution::bst_to_gst2(right, parent);
                if node.right.is_some() {
                    node.val += Solution::left_leave(node.right.clone());
                } else {
                    node.val += parent;
                }
                node.left = Solution::bst_to_gst2(left, node.val);
            }
            return Some(root_node);
        }
        None
    }
}
struct Solution;
fn main() {}
