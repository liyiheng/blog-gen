struct Solution;
fn main() {}
use leetcode::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut all_nodes = HashMap::new();
        let mut parents = HashMap::new();
        let mut queue = vec![root?];
        let mut last_q = queue.clone();
        while !queue.is_empty() {
            let mut tmp = vec![];
            for ele in queue.into_iter() {
                let v = ele.as_ref().borrow();
                all_nodes.insert(v.val, ele.clone());
                let l = v.left.clone();
                let r = v.right.clone();
                if let Some(l) = l {
                    parents.insert(l.as_ref().borrow().val, v.val);
                    tmp.push(l);
                }
                if let Some(r) = r {
                    parents.insert(r.as_ref().borrow().val, v.val);
                    tmp.push(r);
                }
            }
            queue = tmp;
            if !queue.is_empty() {
                last_q = queue.clone();
            }
        }
        use std::collections::HashSet;
        let mut last_q: HashSet<i32> = last_q.iter().map(|v| v.as_ref().borrow().val).collect();
        while last_q.len() > 1 {
            last_q = last_q.iter().map(|n| *parents.get(n).unwrap()).collect();
        }
        let v = last_q.into_iter().next().unwrap();
        all_nodes.get(&v).cloned()
    }
}
