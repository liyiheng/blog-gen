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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut node| match node.next {
            None => Some(node),
            Some(mut next) => {
                let tail = Solution::swap_pairs(next.next);
                node.next = tail;
                next.next = Some(node);
                Some(next)
            }
        })
    }
}

struct Solution;
fn main() {}
