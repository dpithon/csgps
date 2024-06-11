use crate::{Builtin, DictStack, ExecutionStack, Item};

pub struct Stack {
    stack: Vec<Item>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, item: Item) {
        self.stack.push(item);
    }

    pub fn add(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.push(Item::Integer(i1 + i2));
                Ok(())
            }
            (None, _) | (_, None) => Err("'add' stack underflow".to_string()),
            (Some(a), Some(b)) => Err(format!("'add' not implemented: {:?}, {:?}", a, b)),
        }
    }

    pub fn exch(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(i1), Some(i2)) => {
                self.stack.push(i1);
                self.stack.push(i2);
                Ok(())
            }
            (None, _) | (_, None) => Err("'exch' stack underflow".to_string()),
        }
    }

    pub fn roll(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(j)), Some(Item::Integer(n))) => {
                let (len, unsigned_n) = (self.stack.len(), n as usize);
                if n == 0 || n == 1 || j == 0 {
                    Ok(())
                } else if n < 0 {
                    Err("'roll' negative roll range".to_string())
                } else if len < unsigned_n {
                    Err("'roll' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let mut tops: Vec<Item> = self.stack.drain(index..).collect();
                    if j > 0 {
                        tops.rotate_right(j as usize);
                    } else if j < 0 {
                        let j = -j;
                        tops.rotate_left(j as usize);
                    }
                    self.stack.extend(tops);
                    Ok(())
                }
            }
            (Some(_), Some(_)) => Err("'roll', type mismatch".to_string()),
            (None, _) | (_, None) => Err("'roll' stack underflow".to_string()),
        }
    }

    pub fn copy(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(Item::Integer(n)) => {
                let (len, unsigned_n) = (self.stack.len(), n as usize);
                if n < 0 {
                    Err("'copy' negative copy range".to_string())
                } else if len < unsigned_n {
                    Err("'copy' stack too short".to_string())
                } else {
                    let index = len - unsigned_n;
                    let tops: Vec<Item> = Vec::from(&self.stack[index..]);
                    self.stack.extend(tops);
                    Ok(())
                }
            }
            Some(_) => Err("'copy', type mismatch".to_string()),
            None => Err("'copy' stack underflow".to_string()),
        }
    }

    pub fn index(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(Item::Integer(n)) => {
                let (len, unsigned_n) = (self.stack.len(), n as usize);
                if n < 0 {
                    Err("'index' negative index".to_string())
                } else if len <= unsigned_n {
                    Err("'index' stack too short".to_string())
                } else {
                    let index = len - unsigned_n - 1;
                    let element: Item = self.stack[index].clone();
                    self.stack.push(element);
                    Ok(())
                }
            }
            Some(_) => Err("'copy', type mismatch".to_string()),
            None => Err("'copy' stack underflow".to_string()),
        }
    }

    pub fn pop(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err("'pop' stack underflow".to_string()),
        }
    }

    pub fn exec(&mut self, execution_stack: &mut ExecutionStack) -> Result<(), String> {
        match self.stack.pop() {
            Some(Item::Proc(p)) => {
                for proc_item in p.iter().rev() {
                    execution_stack.push(proc_item.clone())
                }
                Ok(())
            }
            Some(i) => {
                self.stack.push(i);
                Ok(())
            }
            None => Err("'pop' stack underflow".to_string()),
        }
    }

    pub fn dup(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(i) => {
                self.stack.push(i.clone());
                self.stack.push(i.clone());
                Ok(())
            }
            None => Err("'dup' stack underflow".to_string()),
        }
    }

    pub fn gt(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Bool(i2 > i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.stack.push(Item::Bool(b2 > b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'gt' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'gt' not implemented".to_string()),
        }
    }

    pub fn eq(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Bool(i2 == i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.stack.push(Item::Bool(b2 == b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'eq' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'eq' not implemented".to_string()),
        }
    }

    pub fn ne(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Bool(i2 != i1));
                Ok(())
            }
            (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
                self.stack.push(Item::Bool(b2 != b1));
                Ok(())
            }
            (None, _) | (_, None) => Err("'ne' stack underflow".to_string()),
            (Some(_), Some(_)) => Err("'ne' not implemented".to_string()),
        }
    }

    pub fn mul(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Integer(i1 * i2));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn sub(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Integer(i2 - i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn div(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Integer(i2 / i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn modulo(&mut self) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
                self.stack.push(Item::Integer(i2 % i1));
                Ok(())
            }
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(_), Some(_)) => Err("not implemented".to_string()),
        }
    }

    pub fn pop_and_print(&mut self) -> Result<(), String> {
        if let Some(item) = self.stack.pop() {
            println!("{item}");
            Ok(())
        } else {
            Err("'=' stack underflow".to_string())
        }
    }

    pub fn print_stack(&mut self) -> Result<(), String> {
        for item in self.stack.iter() {
            print!("{item} ")
        }
        println!("---");
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.stack.clear();
        Ok(())
    }

    pub fn clear_to_mark(&mut self) -> Result<(), String> {
        loop {
            match self.stack.pop() {
                Some(Item::Mark) => return Ok(()),
                Some(_) => (),
                None => return Err("'cleartomark' Mark not found".to_string()),
            }
        }
    }

    pub fn count_to_mark(&mut self) -> Result<(), String> {
        let mut count = 0;
        let mut found = false;
        for item in self.stack.iter().rev() {
            match item {
                Item::Mark => {
                    found = true;
                    break;
                }
                _ => count += 1,
            }
        }
        if found {
            self.stack.push(Item::Integer(count));
            Ok(())
        } else {
            Err("'counttomark' Mark not found".to_string())
        }
    }

    pub fn make_array(&mut self) -> Result<(), String> {
        let mut array: Vec<Item> = Vec::new();

        while let Some(item) = self.stack.pop() {
            match item {
                Item::Mark => {
                    self.stack.push(Item::Array(array));
                    return Ok(());
                }
                item => array.insert(0, item),
            }
        }

        Err("begin mark not found!".to_string())
    }

    pub fn make_proc(&mut self) -> Result<(), String> {
        let mut array: Vec<Item> = Vec::new();

        while let Some(item) = self.stack.pop() {
            match item {
                Item::Mark => {
                    self.stack.push(Item::Proc(array));
                    return Ok(());
                }
                item => array.insert(0, item),
            }
        }

        Err("begin mark not found!".to_string())
    }

    pub fn def(&mut self, dict_stack: &mut DictStack) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (None, _) | (_, None) => Err("stack underflow".to_string()),
            (Some(item), Some(Item::LitName(n))) => {
                dict_stack.def(n, item);
                Ok(())
            }
            _ => Err("wrong argument(s)".to_string()),
        }
    }

    pub fn cond_if(&mut self, execution_stack: &mut ExecutionStack) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Proc(p)), Some(Item::Bool(b))) => {
                if b {
                    for proc_item in p.iter().rev() {
                        execution_stack.push(proc_item.clone())
                    }
                }
                Ok(())
            }
            (Some(a), Some(b)) => Err(format!("'if' wrong argument types {:?} {:?}", a, b)),
            (None, _) | (_, None) => Err("'if' stack underflow".to_string()),
        }
    }

    pub fn cond_ifelse(&mut self, execution_stack: &mut ExecutionStack) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop(), self.stack.pop()) {
            (Some(Item::Proc(pelse)), Some(Item::Proc(pif)), Some(Item::Bool(b))) => {
                if b {
                    for proc_item in pif.iter().rev() {
                        execution_stack.push(proc_item.clone())
                    }
                } else {
                    for proc_item in pelse.iter().rev() {
                        execution_stack.push(proc_item.clone())
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

    pub fn repeat(&mut self, execution_stack: &mut ExecutionStack) -> Result<(), String> {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Proc(p)), Some(Item::Integer(i))) => {
                if i > 1 {
                    execution_stack.push(Item::Builtin(Builtin::Repeat));
                    execution_stack.push(Item::Proc(p.clone()));
                    execution_stack.push(Item::Integer(i - 1));
                }

                for proc_item in p.iter().rev() {
                    execution_stack.push(proc_item.clone())
                }

                Ok(())
            }
            (Some(a), Some(b)) => Err(format!("'repeat' wrong argument types {:?} {:?}", a, b)),
            (None, _) | (_, None) => Err("'repeat' stack underflow".to_string()),
        }
    }
}
