// 剑指 Offer II 046. 二叉树的右侧视图
struct Solution;
use leetcode::TreeNode;
fn main() {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut q = vec![];
        if let Some(v) = root {
            q.push(v);
            while !q.is_empty() {
                ans.push(q.last().unwrap().borrow().val);
                let mut qq = vec![];
                for n in q.iter() {
                    if let Some(l) = n.borrow().left.clone() {
                        qq.push(l);
                    }
                    if let Some(r) = n.borrow().right.clone() {
                        qq.push(r);
                    }
                }
                q = qq
            }
        }
        ans
    }
}
