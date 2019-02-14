// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        let mut stack = vec![];
        let mut result = Box::new(ListNode::new(0));
        let mut tail = &mut result;
        while cur.is_some() {
            for _ in 0..k {
                if cur.is_none() {
                    break;
                }
                let v = cur.as_mut().unwrap();
                cur = &mut v.next;
                stack.push(v.val);
            }
            if stack.len() == k as usize {
                for v in stack.iter().rev() {
                    let next = Box::new(ListNode::new(*v));
                    tail.next = Some(next);
                    tail = tail.next.as_mut().unwrap();
                }
            } else {
                for v in stack.iter() {
                    let next = Box::new(ListNode::new(*v));
                    tail.next = Some(next);
                    tail = tail.next.as_mut().unwrap();
                }
            }
            stack.clear();
        }
        result.next
    }
}

struct Solution;
fn main() {}
