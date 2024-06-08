use clap::Parser;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

// TODO: complete "wrong argument types {:?} ..." messages
// TODO: Stop on error

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"%.*")]
pub enum Token {
    #[token(r"=")]
    PopAndPrint,
    #[token(r"stack")]
    Stack,
    #[regex(r"[-+]?[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Bool(bool),
    #[token(r"counttomark")]
    CountToMark,
    #[token(r"cleartomark")]
    ClearToMark,
    #[token(r"clear")]
    Clear,
    #[token(r"roll")]
    Roll,
    #[token(r"dup")]
    Dup,
    #[token("pop")]
    Pop,
    #[token("exch")]
    Exch,
    #[token(r"add")]
    Add,
    #[token(r"sub")]
    Sub,
    #[token(r"mul")]
    Mul,
    #[token(r"div")]
    Div,
    #[token(r"mod")]
    Mod,
    #[token(r"def")]
    Def,
    #[token(r"gt")]
    Gt,
    #[token(r"eq")]
    Eq,
    #[token(r"ne")]
    Ne,
    // #[token(r"undef")]
    // Undef,
    // #[token(r"for")]
    // For,
    #[token(r"exec")]
    Exec,
    #[token(r"repeat")]
    Repeat,
    #[token(r"ifelse")]
    IfElse,
    #[token(r"if")]
    If,
    #[regex(r"[\[{]|<<")]
    #[token(r"mark")]
    Mark,
    #[token(r"]")]
    Array,
    #[token(r"}")]
    Proc,
    // #[token(r">>")]
    // Dict,
    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice().to_owned())]
    ExeName(String),
    #[regex(r"//[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[2..].to_owned())]
    ImmName(String),
    #[regex(r"/[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[1..].to_owned())]
    LitName(String),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    match execute(&args.filename) {
        Err(e) => println!("an error occured: {e}"),
        Ok(_) => println!("bye."),
    }
}

struct ExecutionStack {
    stack: Vec<Action>,
}

impl ExecutionStack {
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

struct DictStack {
    stack: Vec<HashMap<String, Item>>,
}

impl DictStack {
    pub fn new() -> Self {
        let mut ds = DictStack { stack: Vec::new() };
        ds.stack.push(HashMap::new());
        ds
    }

    pub fn def(&mut self, key: String, val: Item) {
        let mut top = self.stack.pop().unwrap();
        top.insert(key, val);
        self.stack.push(top);
    }

    pub fn get(&self, key: &str) -> Option<Item> {
        let top = self.stack.last().unwrap();
        match top.get(key) {
            None => None,
            Some(item) => Some(item.clone()),
        }
    }
}

#[derive(Debug, Clone)]
enum Item {
    Integer(i64),
    Bool(bool),
    Mark,
    Builtin(Builtin),
    Array(Vec<Item>),
    Proc(Vec<Item>),
    LitName(String),
    ExeName(String),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LitName(n) => write!(f, "LitName({n})"),
            Self::ExeName(n) => write!(f, "ExeName({n})"),
            Self::Integer(i) => write!(f, "Integer({i})"),
            Self::Bool(b) => write!(f, "Bool({b})"),
            Self::Mark => write!(f, "Mark"),
            Self::Builtin(builtin) => write!(f, "{:?}", builtin),
            Self::Array(a) => {
                write!(f, "Array[")?;
                for item in a.iter() {
                    write!(f, "{item},")?;
                }
                write!(f, "]")
            }
            Self::Proc(a) => {
                write!(f, "Proc {{")?;
                for item in a.iter() {
                    write!(f, "{item},")?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Builtin {
    Add,
    Clear,
    Def,
    Div,
    Exec,
    PopAndPrint,
    Dup,
    Eq,
    Ne,
    Exch,
    Gt,
    If,
    IfElse,
    Mod,
    Mul,
    Pop,
    Repeat,
    Roll,
    Sub,
}

enum Action {
    Push(Item),
    PushImmName(String),
    ExecBuiltin(Builtin),
    ClearToMark,
    CountToMark,
    ExecName(String),
    MakeArray,
    MakeProc,
    Stack,
}

fn get_action(token: &Token) -> Action {
    match token {
        Token::Add => Action::ExecBuiltin(Builtin::Add),
        Token::Array => Action::MakeArray,
        Token::Bool(b) => Action::Push(Item::Bool(*b)),
        Token::Clear => Action::ExecBuiltin(Builtin::Clear),
        Token::ClearToMark => Action::ClearToMark,
        Token::CountToMark => Action::CountToMark,
        Token::Def => Action::ExecBuiltin(Builtin::Def),
        Token::Div => Action::ExecBuiltin(Builtin::Div),
        Token::Exec => Action::ExecBuiltin(Builtin::Exec),
        Token::PopAndPrint => Action::ExecBuiltin(Builtin::PopAndPrint),
        Token::Dup => Action::ExecBuiltin(Builtin::Dup),
        Token::Eq => Action::ExecBuiltin(Builtin::Eq),
        Token::Exch => Action::ExecBuiltin(Builtin::Exch),
        Token::ExeName(n) => Action::ExecName(n.clone()),
        Token::Gt => Action::ExecBuiltin(Builtin::Gt),
        Token::Ne => Action::ExecBuiltin(Builtin::Ne),
        Token::If => Action::ExecBuiltin(Builtin::If),
        Token::IfElse => Action::ExecBuiltin(Builtin::IfElse),
        Token::Integer(i) => Action::Push(Item::Integer(*i)),
        Token::LitName(n) => Action::Push(Item::LitName(n.clone())),
        Token::ImmName(n) => Action::PushImmName(n.clone()),
        Token::Mark => Action::Push(Item::Mark),
        Token::Mod => Action::ExecBuiltin(Builtin::Mod),
        Token::Mul => Action::ExecBuiltin(Builtin::Mul),
        Token::Pop => Action::ExecBuiltin(Builtin::Pop),
        Token::Proc => Action::MakeProc,
        Token::Repeat => Action::ExecBuiltin(Builtin::Repeat),
        Token::Roll => Action::ExecBuiltin(Builtin::Roll),
        Token::Sub => Action::ExecBuiltin(Builtin::Sub),
        Token::Stack => Action::Stack,
    }
}

fn exch(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(i1), Some(i2)) => {
            stack.push(i1);
            stack.push(i2);
            Ok(())
        }
        (None, _) | (_, None) => Err("'exch' stack underflow".to_string()),
    }
}

fn roll(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(j)), Some(Item::Integer(n))) => {
            let (len, unsigned_n) = (stack.len(), n as usize);
            if n == 0 || n == 1 || j == 0 {
                Ok(())
            } else if n < 0 {
                Err("'roll' negative roll range".to_string())
            } else if len < unsigned_n {
                Err("'roll' stack too short".to_string())
            } else {
                let index = len - unsigned_n;
                let mut tops: Vec<Item> = stack.drain(index..).collect();
                if j > 0 {
                    tops.rotate_right(j as usize);
                } else if j < 0 {
                    let j = -j;
                    tops.rotate_left(j as usize);
                }
                stack.extend(tops);
                Ok(())
            }
        }
        (Some(_), Some(_)) => Err("'roll', type mismatch".to_string()),
        (None, _) | (_, None) => Err("stack underflow".to_string()),
    }
}

fn pop(stack: &mut Vec<Item>) -> Result<(), String> {
    match stack.pop() {
        Some(_) => Ok(()),
        None => Err("'pop' stack underflow".to_string()),
    }
}

fn exec(stack: &mut Vec<Item>, execution_stack: &mut ExecutionStack) -> Result<(), String> {
    match stack.pop() {
        Some(Item::Proc(p)) => {
            for proc_item in p.iter().rev() {
                execution_stack.push(proc_item.clone())
            }
            Ok(())
        }
        Some(i) => {
            stack.push(i);
            Ok(())
        }
        None => Err("'pop' stack underflow".to_string()),
    }
}

fn dup(stack: &mut Vec<Item>) -> Result<(), String> {
    match stack.pop() {
        Some(i) => {
            stack.push(i.clone());
            stack.push(i.clone());
            Ok(())
        }
        None => Err("'dup' stack underflow".to_string()),
    }
}

fn add(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Integer(i1 + i2));
            Ok(())
        }
        (None, _) | (_, None) => Err("'add' stack underflow".to_string()),
        (Some(a), Some(b)) => Err(format!("'add' not implemented: {:?}, {:?}", a, b)),
    }
}

fn gt(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Bool(i2 > i1));
            Ok(())
        }
        (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
            stack.push(Item::Bool(b2 > b1));
            Ok(())
        }
        (None, _) | (_, None) => Err("'gt' stack underflow".to_string()),
        (Some(_), Some(_)) => Err("'gt' not implemented".to_string()),
    }
}

fn eq(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Bool(i2 == i1));
            Ok(())
        }
        (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
            stack.push(Item::Bool(b2 == b1));
            Ok(())
        }
        (None, _) | (_, None) => Err("'eq' stack underflow".to_string()),
        (Some(_), Some(_)) => Err("'eq' not implemented".to_string()),
    }
}

fn ne(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Bool(i2 != i1));
            Ok(())
        }
        (Some(Item::Bool(b1)), Some(Item::Bool(b2))) => {
            stack.push(Item::Bool(b2 != b1));
            Ok(())
        }
        (None, _) | (_, None) => Err("'ne' stack underflow".to_string()),
        (Some(_), Some(_)) => Err("'ne' not implemented".to_string()),
    }
}

fn mul(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Integer(i1 * i2));
            Ok(())
        }
        (None, _) | (_, None) => Err("stack underflow".to_string()),
        (Some(_), Some(_)) => Err("not implemented".to_string()),
    }
}

fn sub(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Integer(i2 - i1));
            Ok(())
        }
        (None, _) | (_, None) => Err("stack underflow".to_string()),
        (Some(_), Some(_)) => Err("not implemented".to_string()),
    }
}

fn div(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Integer(i2 / i1));
            Ok(())
        }
        (None, _) | (_, None) => Err("stack underflow".to_string()),
        (Some(_), Some(_)) => Err("not implemented".to_string()),
    }
}

fn modulo(stack: &mut Vec<Item>) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(Item::Integer(i1)), Some(Item::Integer(i2))) => {
            stack.push(Item::Integer(i2 % i1));
            Ok(())
        }
        (None, _) | (_, None) => Err("stack underflow".to_string()),
        (Some(_), Some(_)) => Err("not implemented".to_string()),
    }
}

fn pop_and_print(stack: &mut Vec<Item>) -> Result<(), String> {
    if let Some(item) = stack.pop() {
        println!("{item}");
        Ok(())
    } else {
        Err("'=' stack underflow".to_string())
    }
}

fn print_stack(stack: &mut Vec<Item>) -> Result<(), String> {
    for item in stack.iter() {
        print!("{item} ")
    }
    println!("---");
    Ok(())
}

fn clear(stack: &mut Vec<Item>) -> Result<(), String> {
    stack.clear();
    Ok(())
}

fn clear_to_mark(stack: &mut Vec<Item>) -> Result<(), String> {
    loop {
        match stack.pop() {
            Some(Item::Mark) => return Ok(()),
            Some(_) => (),
            None => return Err("'cleartomark' Mark not found".to_string()),
        }
    }
}

fn count_to_mark(stack: &mut Vec<Item>) -> Result<(), String> {
    let mut count = 0;
    let mut found = false;
    for item in stack.iter().rev() {
        match item {
            Item::Mark => {
                found = true;
                break;
            }
            _ => count += 1,
        }
    }
    if found {
        stack.push(Item::Integer(count));
        Ok(())
    } else {
        Err("'counttomark' Mark not found".to_string())
    }
}

fn make_array(stack: &mut Vec<Item>) -> Result<(), String> {
    let mut array: Vec<Item> = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            Item::Mark => {
                stack.push(Item::Array(array));
                return Ok(());
            }
            item => array.insert(0, item),
        }
    }

    Err("begin mark not found!".to_string())
}

fn make_proc(stack: &mut Vec<Item>) -> Result<(), String> {
    let mut array: Vec<Item> = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            Item::Mark => {
                stack.push(Item::Proc(array));
                return Ok(());
            }
            item => array.insert(0, item),
        }
    }

    Err("begin mark not found!".to_string())
}

fn def(stack: &mut Vec<Item>, dict_stack: &mut DictStack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (None, _) | (_, None) => Err("stack underflow".to_string()),
        (Some(item), Some(Item::LitName(n))) => {
            dict_stack.def(n, item);
            Ok(())
        }
        _ => Err("wrong argument(s)".to_string()),
    }
}

fn cond_if(stack: &mut Vec<Item>, execution_stack: &mut ExecutionStack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
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

fn cond_ifelse(stack: &mut Vec<Item>, execution_stack: &mut ExecutionStack) -> Result<(), String> {
    match (stack.pop(), stack.pop(), stack.pop()) {
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
        (_, _, None) | (_, None, _) | (None, _, _) => Err("'ifelse' stack underflow".to_string()),
    }
}

fn repeat(stack: &mut Vec<Item>, execution_stack: &mut ExecutionStack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
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

fn execute(filename: &str) -> Result<(), String> {
    let mut file = match File::open(filename) {
        Err(e) => return Err(format!("error on opening {filename}: {e}")),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("error on loading {filename}: {e}"));
    }

    let mut lex = Token::lexer(&contents);
    let mut execution_stack = ExecutionStack::new();
    let mut dict_stack = DictStack::new();
    let mut stack: Vec<Item> = Vec::new();
    let mut collect_level = 0;

    loop {
        let action = if execution_stack.is_runnable() {
            execution_stack.get_action()
        } else {
            match lex.next() {
                None => break,
                Some(Err(_)) => {
                    return Err(format!("error while parsing {filename}: {}", lex.slice()))
                }
                Some(Ok(token)) => get_action(&token),
            }
        };

        match action {
            Action::Push(Item::Mark) => {
                stack.push(Item::Mark);
                collect_level += 1;
            }
            Action::Push(item) => stack.push(item),
            Action::PushImmName(name) => {
                if let Some(item) = dict_stack.get(&name) {
                    stack.push(item);
                } else {
                    println!("ImmName '{name}' not found");
                }
            }
            Action::ExecBuiltin(builtin) if collect_level == 0 => match builtin {
                Builtin::Add => add(&mut stack),
                Builtin::Clear => clear(&mut stack),
                Builtin::Def => def(&mut stack, &mut dict_stack),
                Builtin::Div => div(&mut stack),
                Builtin::Exec => exec(&mut stack, &mut execution_stack),
                Builtin::PopAndPrint => pop_and_print(&mut stack),
                Builtin::Dup => dup(&mut stack),
                Builtin::Eq => eq(&mut stack),
                Builtin::Ne => ne(&mut stack),
                Builtin::Exch => exch(&mut stack),
                Builtin::Gt => gt(&mut stack),
                Builtin::If => cond_if(&mut stack, &mut execution_stack),
                Builtin::IfElse => cond_ifelse(&mut stack, &mut execution_stack),
                Builtin::Mod => modulo(&mut stack),
                Builtin::Mul => mul(&mut stack),
                Builtin::Pop => pop(&mut stack),
                Builtin::Repeat => repeat(&mut stack, &mut execution_stack),
                Builtin::Roll => roll(&mut stack),
                Builtin::Sub => sub(&mut stack),
            }?,
            Action::Stack => print_stack(&mut stack)?,
            Action::ExecBuiltin(builtin) => stack.push(Item::Builtin(builtin)),
            Action::CountToMark => count_to_mark(&mut stack)?,
            Action::ClearToMark => {
                clear_to_mark(&mut stack)?;
                collect_level -= 1;
            }
            Action::MakeArray => {
                make_array(&mut stack)?;
                collect_level -= 1;
            }
            Action::MakeProc => {
                make_proc(&mut stack)?;
                collect_level -= 1;
            }
            Action::ExecName(n) if collect_level == 0 => {
                if let Some(item) = dict_stack.get(&n) {
                    if let Item::Proc(proc) = item {
                        for proc_item in proc.iter().rev() {
                            execution_stack.push(proc_item.clone());
                        }
                    } else {
                        execution_stack.push(item)
                    }
                } else {
                    println!("ExeName '{n}' not found");
                }
            }
            Action::ExecName(n) => stack.push(Item::ExeName(n)),
        }
    }

    Ok(())
}
