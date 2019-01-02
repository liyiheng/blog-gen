#![allow(dead_code)]
fn main() {
    println!("Kth");
    let mut k = KthLargest::new(7, vec![-10, 1, 3, 1, 4, 10, 3, 9, 4, 5, 1]);
    println!("{:?}", k.heap.data);
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
struct MinHeap<E: Ord> {
    data: Vec<Option<E>>,
    cap: usize,
    len: usize,
}

impl<E: Ord + Clone + Copy> MinHeap<E> {
    fn left(i: usize) -> usize {
        2 * i + 1
    }
    fn right(i: usize) -> usize {
        2 * i + 2
    }

    pub fn new(cap: usize) -> Self {
        MinHeap {
            data: vec![None; cap],
            cap: cap,
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn peak(&self) -> Option<E> {
        if self.len == 0 {
            None
        } else {
            self.data[0]
        }
    }
    pub fn add(&mut self, e: E) {
        if self.cap == 0 {
            return;
        }
        if self.len < self.cap {
            self.data[self.len] = Some(e);
            self.len += 1;
            self.heapify_up();
            return;
        }

        let first = self.data[0];
        if e > first.unwrap() {
            self.data[0] = Some(e);
            self.heapify_down();
        }
    }
    fn heapify_down(&mut self) {
        let mut cur_index = 0;
        while cur_index < self.len {
            let cur = self.data[cur_index].unwrap();
            let left_index = Self::left(cur_index);
            let right_index = Self::right(cur_index);
            if left_index >= self.len && right_index >= self.len {
                return;
            }
            if right_index >= self.len {
                let left = self.data[left_index].unwrap();
                if cur > left {
                    self.data.swap(cur_index, left_index);
                    cur_index = left_index;
                    continue;
                }
            }
            if left_index >= self.len {
                let right = self.data[right_index].unwrap();
                if cur > right {
                    self.data.swap(cur_index, right_index);
                    cur_index = right_index;
                    continue;
                }
            }

            let right = self.data[right_index].unwrap();
            let left = self.data[left_index].unwrap();
            if cur <= right && cur <= left {
                break;
            }
            if cur > right && cur > left {
                if right > left {
                    self.data.swap(cur_index, left_index);
                    cur_index = left_index;
                    continue;
                }
                self.data.swap(cur_index, right_index);
                cur_index = right_index;
                continue;
            }
            if cur > right {
                self.data.swap(cur_index, right_index);
                cur_index = right_index;
                continue;
            }
            self.data.swap(cur_index, left_index);
            cur_index = left_index;
            continue;
        }
    }

    fn heapify_up(&mut self) {
        let mut cur_index = self.len - 1;
        while cur_index > 0 {
            let parent_index = (cur_index - 1) / 2;
            let cur = self.data[cur_index].unwrap();
            let parent = self.data[parent_index].unwrap();
            if cur < parent {
                self.data.swap(cur_index, parent_index);
            }
            cur_index = parent_index;
        }
    }
}

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
