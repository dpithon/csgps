use crate::LitExe;
use crate::Object::{self, Array};

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
        let proc = self.stack.pop().unwrap(); // TODO: unwrap ...
        let object = Array(LitExe::Executable, proc);

        if self.stack.is_empty() {
            Some(object)
        } else {
            self.stack.last_mut().unwrap().push(object);
            None
        }
    }

    pub fn get_proc(&mut self) {}

    pub fn push(&mut self, object: Object) {
        self.stack.last_mut().unwrap().push(object);
    }
}
