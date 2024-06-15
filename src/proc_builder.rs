use crate::Object::{self, Array};
use crate::ObjectMode;

#[derive(Default)]
pub struct ProcBuilder {
    stack: Vec<Vec<Object>>,
}

impl ProcBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_open(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn open(&mut self) {
        self.stack.push(Vec::new());
    }

    pub fn close(&mut self) -> Option<Object> {
        if let Some(proc) = self.stack.pop() {
            let object = Array(ObjectMode::Executable, proc);

            if self.stack.is_empty() {
                Some(object)
            } else {
                self.stack.last_mut().unwrap().push(object);
                None
            }
        } else {
            panic!("unmatched }}");
        }
    }

    pub fn push(&mut self, object: Object) {
        self.stack.last_mut().unwrap().push(object);
    }
}
