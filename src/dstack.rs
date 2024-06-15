use crate::{LitExe, Object, Op};
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<HashMap<String, Object>>,
}

const SYSTEMDICT: [(&str, Object); 24] = [
    ("]", Object::Operator(LitExe::Executable, Op::EndArray)),
    ("add", Object::Operator(LitExe::Executable, Op::Add)),
    ("clear", Object::Operator(LitExe::Executable, Op::Clear)),
    ("copy", Object::Operator(LitExe::Executable, Op::Copy)),
    ("index", Object::Operator(LitExe::Executable, Op::Index)),
    ("def", Object::Operator(LitExe::Executable, Op::Def)),
    ("div", Object::Operator(LitExe::Executable, Op::Div)),
    ("exec", Object::Operator(LitExe::Executable, Op::Exec)),
    ("=", Object::Operator(LitExe::Executable, Op::PopAndPrint)),
    ("dup", Object::Operator(LitExe::Executable, Op::Dup)),
    ("eq", Object::Operator(LitExe::Executable, Op::Eq)),
    ("ne", Object::Operator(LitExe::Executable, Op::Ne)),
    ("exch", Object::Operator(LitExe::Executable, Op::Exch)),
    ("gt", Object::Operator(LitExe::Executable, Op::Gt)),
    ("if", Object::Operator(LitExe::Executable, Op::If)),
    ("ifelse", Object::Operator(LitExe::Executable, Op::IfElse)),
    ("mod", Object::Operator(LitExe::Executable, Op::Mod)),
    ("mul", Object::Operator(LitExe::Executable, Op::Mul)),
    ("pop", Object::Operator(LitExe::Executable, Op::Pop)),
    ("repeat", Object::Operator(LitExe::Executable, Op::Repeat)),
    ("roll", Object::Operator(LitExe::Executable, Op::Roll)),
    ("sub", Object::Operator(LitExe::Executable, Op::Sub)),
    ("load", Object::Operator(LitExe::Executable, Op::Load)),
    ("pstack", Object::Operator(LitExe::Executable, Op::Pstack)),
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
