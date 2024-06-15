use crate::Object;

#[derive(Default)]
pub struct ExecStack {
    stack: Vec<Object>,
}

impl ExecStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn is_runnable(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_object(&mut self) -> Object {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, object: Object) {
        self.stack.push(object);
    }
}
