use leetcode::TreeNode;
struct Solution;
fn main() {
    Solution::level_order_bottom(None);
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = vec![];
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        q.push(root.unwrap());
        while !q.is_empty() {
            let lvl: Vec<i32> = q.iter().map(|v| v.borrow().val).collect();
            ans.push(lvl);
            q = q.into_iter().fold(Default::default(), |mut qq, ele| {
                if let Some(l) = ele.borrow().left.clone() {
                    qq.push(l);
                }
                if let Some(r) = ele.borrow().right.clone() {
                    qq.push(r);
                }
                qq
            });
        }
        ans.reverse();
        ans
    }
}
