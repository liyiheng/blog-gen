use leetcode::TreeNode;
struct Solution;
fn main() {}
type Node = Rc<RefCell<TreeNode>>;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
fn walk(
    ans: &mut Vec<Option<Node>>,
    node: Option<Rc<RefCell<TreeNode>>>,
    del: &HashSet<i32>,
    mut new: bool,
) {
    if node.is_none() {
        return;
    }
    let node = node.unwrap();
    if new && !del.contains(&node.as_ref().borrow().val) {
        ans.push(Some(node.clone()));
        new = false;
    }
    let del_left = node
        .as_ref()
        .borrow()
        .left
        .clone()
        .map(|r| del.contains(&r.as_ref().borrow().val))
        .unwrap_or_default();
    let del_right = node
        .as_ref()
        .borrow()
        .right
        .clone()
        .map(|r| del.contains(&r.as_ref().borrow().val))
        .unwrap_or_default();
    let right = if del_right {
        node.borrow_mut().right.take()
    } else {
        node.as_ref().borrow().right.clone()
    };
    walk(ans, right.clone(), del, del_right);
    if new && !del_right && right.is_some() {
        ans.push(right);
    }

    let left = if del_left {
        node.borrow_mut().left.take()
    } else {
        node.as_ref().borrow().left.clone()
    };
    walk(ans, left.clone(), del, del_left);
    if new && !del_left && left.is_some() {
        ans.push(left);
    }
}
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![];
        let del: HashSet<i32> = to_delete.into_iter().collect();
        walk(&mut ans, root, &del, true);
        ans
    }
}
