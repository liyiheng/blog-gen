struct Solution;
pub fn main() {}
use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let v = root.clone().unwrap().borrow().val;
        Solution::max_diff(root, v, v)
    }

    fn max_diff(root: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
        if root.is_none() {
            return -1;
        }
        let root = root.unwrap();
        let val = root.borrow().val;
        let min = val.min(min);
        let max = val.max(max);
        let l = Solution::max_diff(root.borrow().left.clone(), min, max);
        let r = Solution::max_diff(root.borrow().right.clone(), min, max);
        let d = l.max(r);
        d.max(max - min)
    }
}
