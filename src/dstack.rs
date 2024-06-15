use crate::ObjectMode::*;
use crate::{Object, Op};
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<HashMap<String, Object>>,
}

const SYSTEMDICT: [(&str, Object); 24] = [
    ("]", Object::Operator(Executable, Op::EndArray)),
    ("add", Object::Operator(Executable, Op::Add)),
    ("clear", Object::Operator(Executable, Op::Clear)),
    ("copy", Object::Operator(Executable, Op::Copy)),
    ("index", Object::Operator(Executable, Op::Index)),
    ("def", Object::Operator(Executable, Op::Def)),
    ("div", Object::Operator(Executable, Op::Div)),
    ("exec", Object::Operator(Executable, Op::Exec)),
    ("=", Object::Operator(Executable, Op::PopAndPrint)),
    ("dup", Object::Operator(Executable, Op::Dup)),
    ("eq", Object::Operator(Executable, Op::Eq)),
    ("ne", Object::Operator(Executable, Op::Ne)),
    ("exch", Object::Operator(Executable, Op::Exch)),
    ("gt", Object::Operator(Executable, Op::Gt)),
    ("if", Object::Operator(Executable, Op::If)),
    ("ifelse", Object::Operator(Executable, Op::IfElse)),
    ("mod", Object::Operator(Executable, Op::Mod)),
    ("mul", Object::Operator(Executable, Op::Mul)),
    ("pop", Object::Operator(Executable, Op::Pop)),
    ("repeat", Object::Operator(Executable, Op::Repeat)),
    ("roll", Object::Operator(Executable, Op::Roll)),
    ("sub", Object::Operator(Executable, Op::Sub)),
    ("load", Object::Operator(Executable, Op::Load)),
    ("pstack", Object::Operator(Executable, Op::Pstack)),
];

impl Default for DictStack {
    fn default() -> Self {
        let mut ds = DictStack { stack: Vec::new() };
        ds.stack.push(DictStack::build_systemdict());
        ds.stack.push(HashMap::new()); // userdict
        ds
    }
}

impl DictStack {
    pub fn new() -> Self {
        DictStack::default()
    }

    pub fn def(&mut self, key: String, val: Object) {
        let mut top = self.stack.pop().unwrap();
        top.insert(key, val);
        self.stack.push(top);
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        for dict in self.stack.iter().rev() {
            if let Some(object) = dict.get(key) {
                return Some(object.clone());
            }
        }
        None
    }

    fn build_systemdict() -> HashMap<String, Object> {
        let mut dict = HashMap::new();
        for (name, op) in SYSTEMDICT {
            dict.insert(name.to_string(), op);
        }
        dict
    }
}
