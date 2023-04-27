struct Solution;
use leetcode::TreeNode;
fn main() {}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let mut ans = 0;
                let mut q = VecDeque::new();
                q.push_back(n);
                while !q.is_empty() {
                    let n = q.pop_front().unwrap();
                    ans += 1;
                    match n.clone().borrow().left.clone() {
                        Some(v) => q.push_back(v.clone()),
                        None => {}
                    }
                    match n.clone().borrow().right.clone() {
                        Some(v) => q.push_back(v.clone()),
                        None => {}
                    }
                }
                ans
            }
        }
    }
}
