struct Solution;
use leetcode::TreeNode;
fn main() {}

use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
fn search(parents: &mut Vec<Node>, node: Option<Node>, target: i32) -> bool {
    match node {
        None => false,
        Some(n) => {
            if n.borrow().val == target {
                parents.push(n.clone());
                return true;
            }
            if search(parents, n.borrow().left.clone(), target) {
                parents.push(n.clone());
                return true;
            }
            if search(parents, n.borrow().right.clone(), target) {
                parents.push(n.clone());
                return true;
            }
            false
        }
    }
}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parents = vec![];
        search(&mut parents, root.clone(), p.unwrap().borrow().val);
        let mut parents2 = vec![];
        search(&mut parents2, root.clone(), q.unwrap().borrow().val);
        parents.reverse();
        parents2.reverse();
        let mut ans = parents[0].clone();
        for (a, b) in parents.iter().zip(parents2.iter()) {
            if a == b {
                ans = a.clone();
            } else {
                break;
            }
        }
        Some(ans)
    }
}
