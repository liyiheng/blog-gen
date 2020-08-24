use std::cell::RefCell;
struct MinStack {
    inner: RefCell<Vec<i32>>,
    min_eles: RefCell<Vec<i32>>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            inner: RefCell::new(Vec::new()),
            min_eles: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, x: i32) {
        self.inner.borrow_mut().push(x);
        if self.min_eles.borrow().last().is_none() {
            self.min_eles.borrow_mut().push(x);
        } else {
            let m = *self.min_eles.borrow().last().unwrap_or(&0);
            if m >= x {
                self.min_eles.borrow_mut().push(x);
            }
        }
    }

    fn pop(&self) {
        if let Some(v) = self.inner.borrow_mut().pop() {
            let mut mins = self.min_eles.borrow_mut();
            if *mins.last().unwrap() == v {
                mins.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        self.inner.borrow().last().map(|v| *v).unwrap_or_default()
    }

    fn get_min(&self) -> i32 {
        *self.min_eles.borrow().last().unwrap_or(&0)
    }
}

fn main() {
    let obj = MinStack::new();
    obj.push(10);
    obj.push(1);
    obj.push(2);
    obj.push(3);
    obj.push(-55555);
    println!("{}", obj.get_min());
    println!("{}", obj.top());
    obj.pop();
    println!("{}", obj.get_min());
    println!("{}", obj.top());

    obj.pop();
    println!("{}", obj.get_min());
    println!("{}", obj.top());

    obj.pop();
    println!("{}", obj.get_min());
    println!("{}", obj.top());

    obj.pop();
    println!("{}", obj.get_min());
    println!("{}", obj.top());
}
