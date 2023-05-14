struct Solution;
fn main() {
    Solution::rearrange_barcodes(vec![]);
}
impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for c in barcodes {
            *counter.entry(c).or_default() += 1;
        }
        let mut heap = BinaryHeap::new();
        for (n, cnt) in counter {
            heap.push((cnt, n));
        }
        let mut ans = vec![];
        while !heap.is_empty() {
            let mut a = heap.pop().unwrap_or_default();
            let mut b = heap.pop().unwrap_or_default();
            if a.0 > 0 {
                ans.push(a.1);
                a.0 -= 1;
            }
            if b.0 > 0 {
                ans.push(b.1);
                b.0 -= 1;
            }
            if a.0 > 0 {
                heap.push(a);
            }
            if b.0 > 0 {
                heap.push(b);
            }
        }
        ans
    }
}
