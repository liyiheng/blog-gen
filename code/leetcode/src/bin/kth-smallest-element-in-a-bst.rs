struct Solution;
use leetcode::TreeNode;
fn main() {}

use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
fn search(node: Node, cur: &mut i32, target: i32) -> Option<Node> {
    let left = node.borrow().left.clone();
    if let Some(l) = left {
        let n = search(l, cur, target);
        if n.is_some() {
            return n;
        }
    }
    *cur += 1;

    if *cur == target {
        return Some(node);
    }
    if let Some(right) = node.borrow().right.clone() {
        return search(right, cur, target);
    }
    None
}
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut cur = 0;
        search(root.unwrap(), &mut cur, k).unwrap().borrow().val
    }
}
