#![allow(dead_code)]
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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut cur = &mut head;
        let mut extra = 0;
        while l1.is_some() || l2.is_some() {
            let (v1, next1) = if let Some(n) = l1 {
                (n.val, n.next)
            } else {
                (0, None)
            };
            let (v2, next2) = if let Some(n) = l2 {
                (n.val, n.next)
            } else {
                (0, None)
            };
            let mut v = v1 + v2 + extra;
            extra = 0;
            if v >= 10 {
                extra = 1;
                v -= 10;
            }
            cur.next = Some(Box::new(ListNode::new(v)));
            cur = cur.next.as_mut().unwrap();
            l1 = next1;
            l2 = next2;
        }
        if extra > 0 {
            cur.next = Some(Box::new(ListNode::new(1)));
        }
        head.next
    }
}

struct Solution {}
fn main() {}
