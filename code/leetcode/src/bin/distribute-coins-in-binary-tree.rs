struct Solution;
use leetcode::TreeNode;
fn main() {
    Solution::distribute_coins(None);
}
use std::cell::RefCell;
use std::rc::Rc;
fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }
    let root = root.unwrap();
    let cur = root.borrow().val;
    let (step1, extra1) = dfs(root.borrow().left.clone());
    let (step2, extra2) = dfs(root.borrow().right.clone());
    let cur = cur + extra2 + extra1;
    let extra = cur - 1;
    let step = step1 + step2 + extra.abs();
    (step, extra)
}
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root).0
    }
}
