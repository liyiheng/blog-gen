struct Solution;
use leetcode::TreeNode;
fn main() {
    let a = Rc::new(RefCell::new(TreeNode::new(15)));
    let b = Rc::new(RefCell::new(TreeNode::new(7)));
    let c = Rc::new(RefCell::new(TreeNode::new(9)));
    let mut d = TreeNode::new(20);
    d.left = Some(a);
    d.right = Some(b);
    let d = Rc::new(RefCell::new(d));
    let mut root = TreeNode::new(3);
    root.left = Some(c);
    root.right = Some(d);
    let root = Rc::new(RefCell::new(root));
    dbg!(Solution::min_depth(Some(root)));
}
use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
fn is_leaf(node: &Node) -> bool {
    let x = node.borrow();
    x.left.is_none() && x.right.is_none()
}
fn min_depth(cur: i32, depth: i32, node: Node) -> i32 {
    let depth = depth + 1;
    if depth >= cur {
        return cur;
    }
    if is_leaf(&node) {
        return dbg!((cur).min(depth));
    }
    let a = if let Some(vvv) = node.borrow().left.clone() {
        min_depth(cur, depth, vvv)
    } else {
        i32::MAX
    };
    let cur = cur.min(a);
    let b = if let Some(vvv) = node.borrow().right.clone() {
        min_depth(cur, depth, vvv)
    } else {
        i32::MAX
    };
    a.min(b)
}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            min_depth(i32::MAX, 0, root)
        } else {
            0
        }
    }
}
