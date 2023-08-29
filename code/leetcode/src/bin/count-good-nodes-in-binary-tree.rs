struct Solution;
use leetcode::TreeNode;
fn main() {
    Solution::good_nodes(None);
}
use std::cell::RefCell;
use std::rc::Rc;
fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
    match node {
        None => 0,
        Some(node) => {
            let v = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let cur_cnt = if v >= max { 1 } else { 0 };
            let next_max = max.max(v);
            dfs(left, next_max) + dfs(right, next_max) + cur_cnt
        }
    }
}
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root, i32::MIN)
    }
}
