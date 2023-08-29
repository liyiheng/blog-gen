struct Solution;
fn main() {
    Solution::is_balanced(None);
}
use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
fn height(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
    match root {
        None => (true, 0),
        Some(n) => {
            let (ok, l) = height(n.borrow_mut().left.take());
            let (ok2, r) = height(n.borrow_mut().right.take());
            (ok && ok2 && (l - r).abs() <= 1, l.max(r) + 1)
        }
    }
}
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        height(root).0
    }
}
