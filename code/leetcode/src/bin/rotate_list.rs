#![allow(dead_code)]
struct Solution;
fn main() {
    Solution::rotate_right(Some(Box::new(ListNode { val: 1, next: None })), 1);
}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut size = 0;
        let mut cur = &head;
        while cur.is_some() {
            size += 1;
            cur = &(cur.as_ref().unwrap().next);
        }
        let k = size - k % size;
        if k == size {
            return head;
        }
        let mut cur = &mut head;
        for _ in 1..k {
            cur = &mut (cur.as_mut().unwrap().next);
        }
        let mut tail = cur.as_mut().unwrap().next.take();
        let mut cur = &mut tail;
        loop {
            let mut v = cur.as_mut().unwrap();
            if v.next.is_none() {
                v.next = head;
                break;
            }
            cur = &mut v.next;
        }
        tail
    }
}
