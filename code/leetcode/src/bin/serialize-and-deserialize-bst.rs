struct Solution;
fn main() {
    Codec::new().serialize(None);
    Codec::new().deserialize("".to_owned());
}
use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "None".to_owned(),
            Some(n) => {
                let mut v = n.borrow().val.to_string();
                let left = self.serialize(n.borrow().left.clone());
                let right = self.serialize(n.borrow().right.clone());
                v.push(',');
                v.push_str(&left);
                v.push(',');
                v.push_str(&right);
                v
            }
        }
    }
    fn de(data: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let last = data.pop().unwrap();
        if last == "None" {
            return None;
        }
        let v = last.parse().unwrap();
        let node = Rc::new(RefCell::new(TreeNode::new(v)));
        let left = Codec::de(data);
        let right = Codec::de(data);
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        Some(node)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut data: Vec<&str> = data.split(',').rev().collect();
        Codec::de(&mut data)
    }
}
