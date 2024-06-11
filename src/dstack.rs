use crate::Item;
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<HashMap<String, Item>>,
}

impl DictStack {
    pub fn new() -> Self {
        let mut ds = DictStack { stack: Vec::new() };
        ds.stack.push(HashMap::new());
        ds
    }

    pub fn def(&mut self, key: String, val: Item) {
        let mut top = self.stack.pop().unwrap();
        top.insert(key, val);
        self.stack.push(top);
    }

    pub fn get(&self, key: &str) -> Option<Item> {
        let top = self.stack.last().unwrap();
        match top.get(key) {
            None => None,
            Some(item) => Some(item.clone()),
        }
    }
}
