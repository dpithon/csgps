use crate::{Action, Item};

#[derive(Default)]
pub struct ExecStack {
    stack: Vec<Action>,
}

impl ExecStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn is_runnable(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_action(&mut self) -> Action {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, item: Item) {
        match item {
            Item::Bool(_) => self.stack.push(Action::Push(item)),
            Item::Integer(_) => self.stack.push(Action::Push(item)),
            Item::Builtin(b) => self.stack.push(Action::ExecBuiltin(b)),
            Item::ExeName(e) => self.stack.push(Action::ExecName(e)),
            Item::Array(a) => self.stack.push(Action::Push(Item::Array(a.clone()))),
            Item::Proc(p) => self.stack.push(Action::Push(Item::Proc(p.clone()))),
            Item::LitName(l) => self.stack.push(Action::Push(Item::LitName(l.clone()))),
            _ => unimplemented!(),
        }
    }
}
