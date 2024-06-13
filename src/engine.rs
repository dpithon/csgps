use crate::Action;
use crate::Builtin;
use crate::DictStack;
use crate::ExecStack;
use crate::Item;
use crate::{get_action, Token};

use logos::Logos;

use std::fs::File;
use std::io::prelude::*;

use std::cmp::Ordering;

pub struct Engine {
    exec_stack: ExecStack,
    dict_stack: DictStack,
    main_stack: Vec<Item>,
    nested_level: i32,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            exec_stack: ExecStack::new(),
            dict_stack: DictStack::new(),
            main_stack: Vec::new(),
            nested_level: 0,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            exec_stack: ExecStack::new(),
            dict_stack: DictStack::new(),
            main_stack: Vec::new(),
            nested_level: 0,
        }
    }

    pub fn get_stack_size(&self) -> usize {
        self.main_stack.len()
    }

    pub fn process_execution_stack(&mut self) -> Result<(), String> {
        while self.exec_stack.is_runnable() {
            let action = self.exec_stack.get_action();
            self.execute_action(action)?;
        }
        Ok(())
    }

    pub fn execute_string(&mut self, contents: &str) -> Result<(), String> {
        let mut lex = Token::lexer(contents);

        loop {
            self.process_execution_stack()?;

            match lex.next() {
                Some(Ok(token)) => self.execute_action(get_action(&token))?,
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

    pub fn execute_action(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Push(Item::Mark) => {
                self.main_stack.push(Item::Mark);
                self.nested_level += 1;
                Ok(())
            }
            Action::Push(item) => {
                self.main_stack.push(item);
                Ok(())
            }
            Action::PushImmName(name) => {
                if let Some(item) = self.dict_stack.get(&name) {
                    self.main_stack.push(item);
                    Ok(())
                } else {
                    Err("ImmName '{name}' not found".to_string())
                }
            }
            Action::ExecBuiltin(builtin) if self.nested_level == 0 => match builtin {
                Builtin::Add => self.add(),
                Builtin::Clear => self.clear(),
                Builtin::Copy => self.copy(),
                Builtin::Index => self.index(),
                Builtin::Def => self.def(),
                Builtin::Div => self.div(),
                Builtin::Exec => self.exec(),
                Builtin::PopAndPrint => self.pop_and_print(),
                Builtin::Dup => self.dup(),
                Builtin::Eq => self.eq(),
                Builtin::Ne => self.ne(),
                Builtin::Exch => self.exch(),
                Builtin::Gt => self.gt(),
                Builtin::If => self.cond_if(),
                Builtin::IfElse => self.cond_ifelse(),
                Builtin::Mod => self.modulo(),
                Builtin::Mul => self.mul(),
                Builtin::Pop => self.pop(),
                Builtin::Repeat => self.repeat(),
                Builtin::Roll => self.roll(),
                Builtin::Sub => self.sub(),
            },
            Action::Stack => self.print_stack(),
            Action::ExecBuiltin(builtin) => {
                self.main_stack.push(Item::Builtin(builtin));
                Ok(())
            }
            Action::CountToMark => self.count_to_mark(),
            Action::ClearToMark => {
                self.nested_level -= 1;
                self.clear_to_mark()
            }
            Action::MakeArray => {
                self.nested_level -= 1;
                self.make_array()
            }
            Action::MakeProc => {
                self.nested_level -= 1;
                self.make_proc()
            }
            Action::ExecName(n) if self.nested_level == 0 => {
                if let Some(item) = self.dict_stack.get(&n) {
                    if let Item::Proc(proc) = item {
                        for proc_item in proc.iter().rev() {
                            self.exec_stack.push(proc_item.clone());
                        }
                    } else {
                        self.exec_stack.push(item);
                    }
                    Ok(())
                } else {
                    Err(format!("ExeName '{n}' not found"))
                }
            }
            Action::ExecName(n) => {
                self.main_stack.push(Item::ExeName(n));
                Ok(())
            }
        }
    }

    pub fn add(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Integer(i1 + i2));
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
            (Some(Item::Integer(j)), Some(Item::Integer(n))) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n == 0 || n == 1 {
                    Ok(())
                } else if n < 0 {
                    Err("'roll' negative roll range".to_string())
                } else if len < unsigned_n {
                    Err("'roll' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let mut tops: Vec<Item> = self.main_stack.drain(index..).collect();
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
            Some(Item::Integer(n)) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n < 0 {
                    Err("'copy' negative copy range".to_string())
                } else if len < unsigned_n {
                    Err("'copy' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let tops: Vec<Item> = Vec::from(&self.main_stack[index..]);
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
            Some(Item::Integer(n)) => {
                let (len, unsigned_n) = (self.main_stack.len(), n as usize);
                if n < 0 {
                    Err("'index' negative index".to_string())
                } else if len <= unsigned_n {
                    Err("'index' stack too short".to_string())
                } else {
                    let index = len - unsigned_n - 1;
                    let element: Item = self.main_stack[index].clone();
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
            Some(Item::Proc(p)) => {
                for proc_item in p.iter().rev() {
                    self.exec_stack.push(proc_item.clone())
                }
                Ok(())
            }
            Some(i) => {
                self.main_stack.push(i);
                Ok(())
            }
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
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Bool(i2 > i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.main_stack.push(Item::Bool(b2 & !b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'gt' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'gt' not implemented".to_string()),
        }
    }

    pub fn eq(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Bool(i2 == i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.main_stack.push(Item::Bool(b2 == b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'eq' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'eq' not implemented".to_string()),
        }
    }

    pub fn ne(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Bool(i2 != i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.main_stack.push(Item::Bool(b2 != b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'ne' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'ne' not implemented".to_string()),
        }
    }

    pub fn mul(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Integer(i1 * i2));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn sub(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Integer(i2 - i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn div(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Integer(i2 / i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn modulo(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.main_stack.push(Item::Integer(i2 % i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn pop_and_print(&mut self) -> Result<(), String> {
        if let Some(item) = self.main_stack.pop() {
            println!("{item}");
            Ok(())
        } else {
            Err("'=' stack underflow".to_string())
        }
    }

    pub fn print_stack(&mut self) -> Result<(), String> {
        for item in self.main_stack.iter() {
            print!("{item} ")
        }
        println!("---");
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.main_stack.clear();
        Ok(())
    }

    pub fn clear_to_mark(&mut self) -> Result<(), String> {
        loop {
            match self.main_stack.pop() {
                Some(Item::Mark) => return Ok(()),
                Some(_) => (),
                None => return Err("'cleartomark' Mark not found".to_string()),
            }
        }
    }

    pub fn count_to_mark(&mut self) -> Result<(), String> {
        let mut count = 0;
        let mut found = false;
        for item in self.main_stack.iter().rev() {
            match item {
                Item::Mark => {
                    found = true;
                    break;
                }
                _ => count += 1,
            }
        }
        if found {
            self.main_stack.push(Item::Integer(count));
            Ok(())
        } else {
            Err("'counttomark' Mark not found".to_string())
        }
    }

    pub fn make_array(&mut self) -> Result<(), String> {
        let mut array: Vec<Item> = Vec::new();

        while let Some(item) = self.main_stack.pop() {
            match item {
                Item::Mark => {
                    self.main_stack.push(Item::Array(array));
                    return Ok(());
                }
                item => array.insert(0, item),
            }
        }

        Err("begin mark not found!".to_string())
    }

    pub fn make_proc(&mut self) -> Result<(), String> {
        let mut array: Vec<Item> = Vec::new();

        while let Some(item) = self.main_stack.pop() {
            match item {
                Item::Mark => {
                    self.main_stack.push(Item::Proc(array));
                    return Ok(());
                }
                item => array.insert(0, item),
            }
        }

        Err("begin mark not found!".to_string())
    }

    pub fn def(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(item), Some(Item::LitName(n))) => {
                self.dict_stack.def(n, item);
                Ok(())
            }
            _ => Err("wrong argument(s)".to_string()),
        }
    }

    pub fn cond_if(&mut self) -> Result<(), String> {
        match (self.main_stack.pop(), self.main_stack.pop()) {
            (Some(Item::Proc(p)), Some(Item::Bool(b))) => {
                if b {
                    for proc_item in p.iter().rev() {
                        self.exec_stack.push(proc_item.clone())
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
            (Some(Item::Proc(pelse)), Some(Item::Proc(pif)), Some(Item::Bool(b))) => {
                if b {
                    for proc_item in pif.iter().rev() {
                        self.exec_stack.push(proc_item.clone())
                    }
                } else {
                    for proc_item in pelse.iter().rev() {
                        self.exec_stack.push(proc_item.clone())
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
            (Some(Item::Proc(p)), Some(Item::Integer(i))) => {
                if i > 1 {
                    self.exec_stack.push(Item::Builtin(Builtin::Repeat));
                    self.exec_stack.push(Item::Proc(p.clone()));
                    self.exec_stack.push(Item::Integer(i - 1));
                }

                for proc_item in p.iter().rev() {
                    self.exec_stack.push(proc_item.clone())
                }

                Ok(())
            }
            (Some(a), Some(b)) => Err(format!("'repeat' wrong argument types {:?} {:?}", a, b)),
            (None, _) | (_, None) => Err("'repeat' stack underflow".to_string()),
        }
    }
}
