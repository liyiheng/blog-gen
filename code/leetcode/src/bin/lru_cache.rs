fn main() {
    let mut obj = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    let ret_1 = obj.get(1);
    let ret_3 = obj.get(3);
    let ret_4 = obj.get(4);
    println!("{},{},{}", ret_1, ret_3, ret_4);
}

use std::collections::HashMap;
struct LRUCache {
    cap: usize,
    map: HashMap<i32, i32>,
    list: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity as usize,
            map: HashMap::new(),
            list: vec![],
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let v = {
            let value = self.map.get(&key).unwrap_or(&(-1));
            *value
        };
        if v != -1 {
            self.refresh_key(key);
        }
        v
    }
    fn refresh_key(&mut self, key: i32) {
        let mut index = 0;
        for i in 0..self.list.len() {
            if self.list[i] == key {
                index = i;
                break;
            }
        }
        self.list.remove(index);
        self.list.insert(0, key);
    }
    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.refresh_key(key)
        } else {
            self.list.insert(0, key);
        }
        self.map.insert(key, value);
        if self.list.len() > self.cap {
            let oldest = self.list.pop();
            if let Some(k) = oldest {
                self.map.remove(&k);
            }
        }
    }
}
