struct Solution;
fn main() {}
use leetcode::ListNode;
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut v = vec![];
        while head.is_some() {
            let node = head.unwrap();
            v.push(node.val);
            head = node.next;
        }
        let l = v.len();
        for i in 0..(l / 2) {
            if v[i] != v[l - 1 - i] {
                return false;
            }
        }
        true
    }
}
