use crate::ObjectMode::*;
use crate::{Object, Operator::*};
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<HashMap<String, Object>>,
}

const SYSTEMDICT: [(&str, Object); 26] = [
    ("]", Object::Operator(Executable, EndArray)),
    ("=", Object::Operator(Executable, PopAndPrint)),
    ("add", Object::Operator(Executable, Add)),
    ("clear", Object::Operator(Executable, Clear)),
    ("cleartomark", Object::Operator(Executable, ClearToMark)),
    ("copy", Object::Operator(Executable, Copy)),
    ("counttomark", Object::Operator(Executable, CountToMark)),
    ("def", Object::Operator(Executable, Def)),
    ("div", Object::Operator(Executable, Div)),
    ("dup", Object::Operator(Executable, Dup)),
    ("eq", Object::Operator(Executable, Eq)),
    ("exch", Object::Operator(Executable, Exch)),
    ("exec", Object::Operator(Executable, Exec)),
    ("gt", Object::Operator(Executable, Gt)),
    ("ifelse", Object::Operator(Executable, IfElse)),
    ("if", Object::Operator(Executable, If)),
    ("index", Object::Operator(Executable, Index)),
    ("load", Object::Operator(Executable, Load)),
    ("mod", Object::Operator(Executable, Mod)),
    ("mul", Object::Operator(Executable, Mul)),
    ("ne", Object::Operator(Executable, Ne)),
    ("pop", Object::Operator(Executable, Pop)),
    ("pstack", Object::Operator(Executable, Pstack)),
    ("repeat", Object::Operator(Executable, Repeat)),
    ("roll", Object::Operator(Executable, Roll)),
    ("sub", Object::Operator(Executable, Sub)),
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
        eprintln!("register {key}:{val}");
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
