struct Solution;
use leetcode::TreeNode;
fn main() {}
use std::cell::RefCell;
use std::rc::Rc;
fn dfs(cur: i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        let root = root.borrow();
        let cur = cur * 10 + root.val;
        let is_leaf = root.left.is_none() && root.right.is_none();
        if is_leaf {
            return cur;
        }
        return dfs(cur, root.left.clone()) + dfs(cur, root.right.clone());
    }
    0
}
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(0, root)
    }
}
