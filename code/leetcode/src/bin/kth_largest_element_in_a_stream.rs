#![allow(dead_code)]
fn main() {
    println!("Kth");
    let mut k = KthLargest::new(7, vec![-10, 1, 3, 1, 4, 10, 3, 9, 4, 5, 1]);
    println!(
        "{},{},{},{},{},{}",
        k.add(3),
        k.add(2),
        k.add(3),
        k.add(1),
        k.add(2),
        k.add(4)
    );
}
mod min_heap;
use crate::min_heap::MinHeap;
struct KthLargest {
    k: i32,
    heap: MinHeap<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k = KthLargest {
            k: k,
            heap: MinHeap::new(k as usize),
        };
        for i in nums {
            k.heap.add(i);
        }
        k
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.add(val);
        self.heap.peak().unwrap()
    }
}
