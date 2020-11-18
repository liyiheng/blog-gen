struct OrderedStream {
    ptr: usize,
    data: Vec<Option<String>>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            ptr: 0,
            data: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.data[id as usize - 1] = Some(value);
        if self.ptr + 1 == id as usize {
            let result: Vec<String> = self.data[self.ptr..]
                .iter()
                .take_while(|e| e.is_some())
                .cloned()
                .map(|o| o.unwrap())
                .collect();
            self.ptr += result.len();
            return result;
        }
        vec![]
    }
}
fn main() {}
