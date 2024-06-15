use crate::DictStack;
use crate::ExecStack;
use crate::Object;
use crate::ObjectMode::*;
use crate::Op;
use crate::ProcBuilder;
use crate::Token;

use logos::Logos;

use std::fs::File;
use std::io::prelude::*;

use std::cmp::Ordering;

pub struct Engine {
    exec_stack: ExecStack,
    dict_stack: DictStack,
    main_stack: Vec<Object>,
    proc_builder: ProcBuilder,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            exec_stack: ExecStack::new(),
            dict_stack: DictStack::new(),
            main_stack: Vec::new(),
            proc_builder: ProcBuilder::new(),
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine::default()
    }

    pub fn get_stack_size(&self) -> usize {
        self.main_stack.len()
    }

    pub fn translate_to_object(&self, token: Token) -> Object {
        match token {
            Token::Bool(b) => Object::Bool(b),
            Token::Real(r) => Object::Real(r),
            Token::Integer(i) => Object::Integer(i),
            Token::ExeName(n) => Object::Name(Executable, n),
            Token::LitName(n) => Object::Name(Literal, n),
            Token::Mark => Object::Mark,
            _ => panic!("Token not expected {:?}", token),
        }
    }

    pub fn process_execution_stack(&mut self) -> Result<(), String> {
        while self.exec_stack.is_runnable() {
            let object = self.exec_stack.get_object();
            self.process_object(object)?;
        }
        Ok(())
    }

    pub fn execute_string(&mut self, contents: &str) -> Result<(), String> {
        let mut lex = Token::lexer(contents);

        loop {
            self.process_execution_stack()?;

            match lex.next() {
                Some(Ok(Token::BeginProc)) => self.proc_builder.open(),
                Some(Ok(Token::EndProc)) => {
                    if let Some(proc) = self.proc_builder.close() {
                        self.main_stack.push(proc);
                    }
                }
                Some(Ok(Token::ImmName(name))) => {
                    let response = self.dict_stack.get(&name);
                    match response {
                        Some(object) => {
                            if self.proc_builder.is_open() {
                                self.proc_builder.push(object);
                            } else {
                                self.main_stack.push(object);
                            }
                        }
                        None => {
                            todo!("undefined error")
                        }
                    }
                }
                Some(Ok(token)) => {
                    let object = self.translate_to_object(token);
                    if self.proc_builder.is_open() {
                        self.proc_builder.push(object);
                    } else {
                        self.process_object(object)?;
                    }
                }
                Some(Err(_)) => return Err(format!("parse error: {}", lex.slice())),
                None => return Ok(()),
            };
        }
    }

    pub fn execute_file(&mut self, filename: &str) -> Result<(), String> {
        let mut file = match File::open(filename) {
            Err(e) => return Err(format!("error on opening {filename}: {e}")),
            Ok(file) => file,
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("error on loading {filename}: {e}"));
        }

        self.execute_string(&contents)
    }

    pub fn enter_repl(&mut self) {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        loop {
            let readline = rl.readline(&format!("csg-PS [{}]> ", self.get_stack_size()));
            match readline {
                Ok(line) => {
                    if let Err(e) = self.execute_string(&line) {
                        println!("Error : {e}");
                    }
                }
                Err(_) => break,
            };
        }
    }

    pub fn run_operator(&mut self, op: Op) -> Result<(), String> {
        use Op::*;

        match op {
            Add => self.add(),
            Clear => self.clear(),
            Copy => self.copy(),
            Index => self.index(),
            Def => self.def(),
            Div => self.div(),
            Exec => self.exec(),
            PopAndPrint => self.pop_and_print(),
            Dup => self.dup(),
            Eq => self.eq(),
            Ne => self.ne(),
            Exch => self.exch(),
            Gt => self.gt(),
            If => self.cond_if(),
            IfElse => self.cond_ifelse(),
            Mod => self.modulo(),
            Mul => self.mul(),
            Pop => self.pop(),
            Repeat => self.repeat(),
            Roll => self.roll(),
            Sub => self.sub(),
            Load => self.load(),
            Pstack => self.pstack(),
            EndArray => {
                if self.proc_builder.is_open() {
                    self.proc_builder
                        .push(Object::Operator(Executable, EndArray));
                    Ok(())
                } else {
                    let array = self.build_array()?;
                    self.main_stack.push(array);
                    Ok(())
                }
            }
        }
    }

    pub fn process_object(&mut self, object: Object) -> Result<(), String> {
        use Object::*;

        match object {
            Name(Executable, name) => {
                if let Some(obj) = self.dict_stack.get(&name) {
                    match obj {
                        Array(Executable, proc) => {
                            for proc_object in proc.iter().rev() {
                                self.exec_stack.push(proc_object.clone()); // TODO: New Proc Struct
                            }
                            Ok(())
                        }
                        Operator(Executable, op) => {
                            self.run_operator(op)?;
                            Ok(())
                        }
                        other => {
                            self.exec_stack.push(other);
                            Ok(())
                        }
                    }
                } else {
                    Err(format!("ExeName '{name}' not found"))
                }
            }
            Operator(Executable, op) => self.run_operator(op),
            other => {
                self.main_stack.push(other.clone());
                Ok(())
            }
        }
    }

    pub fn add(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Integer(i1 + i2));
                Ok(())
            }
            (None, _) | (_, None) => Err("'add' stack underflow".to_string()),
            (Some(a), Some(b)) => Err(format!("'add' not implemented: {:?}, {:?}", a, b)),
        }
    }

    pub fn exch(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(i1), Some(i2)) => {
                self.main_stack.push(i1);
                self.main_stack.push(i2);
                Ok(())
            }
            (None, _) | (_, None) => Err("'exch' stack underflow".to_string()),
        }
    }

    pub fn roll(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(j)), Some(Object::Integer(n))) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n == 0 || n == 1 {
                    Ok(())
                } else if n < 0 {
                    Err("'roll' negative roll range".to_string())
                } else if len < unsigned_n {
                    Err("'roll' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let mut tops: Vec<Object> = self.main_stack.drain(index..).collect();
                    match j.cmp(&0) {
                        Ordering::Less => {
                            let j = -j;
                            tops.rotate_left(j as usize);
                        }
                        Ordering::Greater => tops.rotate_right(j as usize),
                        Ordering::Equal => return Ok(()),
                    }
                    self.main_stack.extend(tops);
                    Ok(())
                }
            }
            (Some(_), Some(_)) => Err("'roll', type mismatch".to_string()),
            (None, _) | (_, None) => Err("'roll' stack underflow".to_string()),
        }
    }

    pub fn copy(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(Object::Integer(n)) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n < 0 {
                    Err("'copy' negative copy range".to_string())
                } else if len < unsigned_n {
                    Err("'copy' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let tops: Vec<Object> = Vec::from(&self.main_stack[index..]);
                    self.main_stack.extend(tops);
                    Ok(())
                }
            }
            Some(_) => Err("'copy', type mismatch".to_string()),
            None => Err("'copy' stack underflow".to_string()),
        }
    }

    pub fn index(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(Object::Integer(n)) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n < 0 {
                    Err("'index' negative index".to_string())
                } else if len <= unsigned_n {
                    Err("'index' stack too short".to_string())
                } else {
                    let index = len - unsigned_n - 1;
                    let element: Object = self.main_stack[index].clone();
                    self.main_stack.push(element);
                    Ok(())
                }
            }
            Some(_) => Err("'copy', type mismatch".to_string()),
            None => Err("'copy' stack underflow".to_string()),
        }
    }

    pub fn pop(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(_) => Ok(()),
            None => Err("'pop' stack underflow".to_string()),
        }
    }

    pub fn exec(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(Object::Array(Executable, p)) => {
                for proc_object in p.iter().rev() {
                    self.exec_stack.push(proc_object.clone())
                }
                Ok(())
            }
            Some(o) => self.process_object(o),
            None => Err("'pop' stack underflow".to_string()),
        }
    }

    pub fn dup(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(i) => {
                self.main_stack.push(i.clone());
                self.main_stack.push(i.clone());
                Ok(())
            }
            None => Err("'dup' stack underflow".to_string()),
        }
    }

    pub fn gt(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Bool(i2 > i1));
                Ok(())
            }
            (Some(Object::Bool(b1)), Some(Object::Bool(b2))) => {
                self.main_stack.push(Object::Bool(b2 & !b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'gt' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'gt' not implemented".to_string()),
        }
    }

    pub fn eq(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Bool(i2 == i1));
                Ok(())
            }
            (Some(Object::Bool(b1)), Some(Object::Bool(b2))) => {
                self.main_stack.push(Object::Bool(b2 == b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'eq' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'eq' not implemented".to_string()),
        }
    }

    pub fn ne(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Bool(i2 != i1));
                Ok(())
            }
            (Some(Object::Bool(b1)), Some(Object::Bool(b2))) => {
                self.main_stack.push(Object::Bool(b2 != b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'ne' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'ne' not implemented".to_string()),
        }
    }

    pub fn mul(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Integer(i1 * i2));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn sub(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Integer(i2 - i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn div(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Integer(i2 / i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn modulo(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Integer(i1)), Some(Object::Integer(i2))) => {
                self.main_stack.push(Object::Integer(i2 % i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn pop_and_print(&mut self) -> Result<(), String> {
        if let Some(object) = self.main_stack.pop() {
            println!("{object}");
            Ok(())
        } else {
            Err("'=' stack underflow".to_string())
        }
    }

    pub fn pstack(&mut self) -> Result<(), String> {
        for object in self.main_stack.iter().rev() {
            print!("{object} ")
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.main_stack.clear();
        Ok(())
    }

    pub fn clear_to_mark(&mut self) -> Result<(), String> {
        loop {
            match self.main_stack.pop() {
                Some(Object::Mark) => return Ok(()),
                Some(_) => (),
                None => return Err("'cleartomark' Mark not found".to_string()),
            }
        }
    }

    pub fn count_to_mark(&mut self) -> Result<(), String> {
        let mut count = 0;
        let mut found = false;
        for object in self.main_stack.iter().rev() {
            match object {
                Object::Mark => {
                    found = true;
                    break;
                }
                _ => count += 1,
            }
        }
        if found {
            self.main_stack.push(Object::Integer(count));
            Ok(())
        } else {
            Err("'counttomark' Mark not found".to_string())
        }
    }

    pub fn build_array(&mut self) -> Result<Object, String> {
        let mut array: Vec<Object> = Vec::new();

        while let Some(object) = self.main_stack.pop() {
            match object {
                Object::Mark => return Ok(Object::Array(Literal, array)),
                object => array.insert(0, object),
            }
        }

        Err("begin mark not found!".to_string())
    }

    pub fn def(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(object), Some(Object::Name(Literal, n))) => {
                self.dict_stack.def(n, object);
                Ok(())
            }
            _ => Err("wrong argument(s)".to_string()),
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        match self.main_stack.pop() {
            Some(Object::Name(Literal, name)) => {
                if let Some(object) = self.dict_stack.get(&name) {
                    self.main_stack.push(object);
                    Ok(())
                } else {
                    Err("name not found: {name".to_string())
                }
            }
            _ => Err("wrong argument".to_string()),
        }
    }

    pub fn cond_if(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Array(Executable, p)), Some(Object::Bool(b))) => {
                if b {
                    for proc_object in p.iter().rev() {
                        self.exec_stack.push(proc_object.clone())
                    }
                }
                Ok(())
            }
            (Some(a), Some(b)) => Err(format!("'if' wrong argument types {:?} {:?}", a, b)),
            (None, _) | (_, None) => Err("'if' stack underflow".to_string()),
        }
    }

    pub fn cond_ifelse(&mut self) -> Result<(), String> {
        match (
            self.main_stack.pop(),
            self.main_stack.pop(),
            self.main_stack.pop(),
        ) {
            (
                Some(Object::Array(Executable, pelse)),
                Some(Object::Array(Executable, pif)),
                Some(Object::Bool(b)),
            ) => {
                if b {
                    for proc_object in pif.iter().rev() {
                        self.exec_stack.push(proc_object.clone())
                    }
                } else {
                    for proc_object in pelse.iter().rev() {
                        self.exec_stack.push(proc_object.clone())
                    }
                }
                Ok(())
            }
            (Some(a), Some(b), Some(c)) => Err(format!(
                "'ifelse' wrong argument types {:?} {:?} {:?}",
                a, b, c
            )),
            (_, _, None) | (_, None, _) | (None, _, _) => {
                Err("'ifelse' stack underflow".to_string())
            }
        }
    }

    pub fn repeat(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Object::Array(Executable, p)), Some(Object::Integer(i))) => {
                if i > 1 {
                    self.exec_stack
                        .push(Object::Operator(Executable, Op::Repeat));
                    self.exec_stack.push(Object::Array(Executable, p.clone()));
                    self.exec_stack.push(Object::Integer(i - 1));
                }

                for proc_object in p.iter().rev() {
                    self.exec_stack.push(proc_object.clone())
                }

                Ok(())
            }
            (Some(a), Some(b)) => Err(format!("'repeat' wrong argument types {:?} {:?}", a, b)),
            (None, _) | (_, None) => Err("'repeat' stack underflow".to_string()),
        }
    }
}
