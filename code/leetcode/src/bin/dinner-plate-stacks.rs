fn main() {}

use std::collections::BTreeSet;
struct DinnerPlates {
    not_full: BTreeSet<usize>,
    cap: usize,
    stacks: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        DinnerPlates {
            not_full: Default::default(),
            cap: capacity as usize,
            stacks: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        let mut stack_idx = self
            .not_full
            .iter()
            .next()
            .cloned()
            .unwrap_or(self.stacks.len());
        while self.stacks.get(stack_idx).is_none() {
            self.stacks.push(vec![]);
        }
        if self.stacks.get(stack_idx).unwrap().len() == self.cap {
            stack_idx += 1;
            self.stacks.push(vec![]);
        }
        self.stacks[stack_idx].push(val);
        self.update(stack_idx);
    }

    fn pop(&mut self) -> i32 {
        while self.stacks.len() > 0 {
            if self.stacks.last().unwrap().is_empty() {
                self.stacks.pop();
            } else {
                let v = self.stacks.last_mut().unwrap().pop().unwrap();
                self.update(self.stacks.len() - 1);
                return v;
            }
        }
        -1
    }
    fn update(&mut self, idx: usize) {
        if idx >= self.stacks.len() {
            self.not_full.remove(&idx);
        } else {
            let full = self.stacks[idx].len() == self.cap;
            if full {
                self.not_full.remove(&idx);
            } else {
                self.not_full.insert(idx);
            }
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index >= self.stacks.len() {
            return -1;
        }
        let v = self
            .stacks
            .get_mut(index)
            .map(|v| v.pop())
            .flatten()
            .unwrap_or(-1);
        if self.stacks[index].len() < self.cap {
            self.not_full.insert(index);
        }
        v
    }
}
