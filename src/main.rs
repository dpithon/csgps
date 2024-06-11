use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

// TODO: complete "wrong argument types {:?} ..." messages
// TODO: Stop on error

use logos::Logos;

mod builtin;
mod dstack;
mod item;
mod stack;
mod xstack;

use builtin::Builtin;
use dstack::DictStack;
use item::Item;
use stack::Stack;
use xstack::ExecutionStack;

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
    #[token(r"copy")]
    Copy,
    #[token(r"index")]
    Index,
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
        Token::Copy => Action::ExecBuiltin(Builtin::Copy),
        Token::Index => Action::ExecBuiltin(Builtin::Index),
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
    let mut stack = Stack::new();
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
                Builtin::Add => stack.add(),
                Builtin::Clear => stack.clear(),
                Builtin::Copy => stack.copy(),
                Builtin::Index => stack.index(),
                Builtin::Def => stack.def(&mut dict_stack),
                Builtin::Div => stack.div(),
                Builtin::Exec => stack.exec(&mut execution_stack),
                Builtin::PopAndPrint => stack.pop_and_print(),
                Builtin::Dup => stack.dup(),
                Builtin::Eq => stack.eq(),
                Builtin::Ne => stack.ne(),
                Builtin::Exch => stack.exch(),
                Builtin::Gt => stack.gt(),
                Builtin::If => stack.cond_if(&mut execution_stack),
                Builtin::IfElse => stack.cond_ifelse(&mut execution_stack),
                Builtin::Mod => stack.modulo(),
                Builtin::Mul => stack.mul(),
                Builtin::Pop => stack.pop(),
                Builtin::Repeat => stack.repeat(&mut execution_stack),
                Builtin::Roll => stack.roll(),
                Builtin::Sub => stack.sub(),
            }?,
            Action::Stack => stack.print_stack()?,
            Action::ExecBuiltin(builtin) => stack.push(Item::Builtin(builtin)),
            Action::CountToMark => stack.count_to_mark()?,
            Action::ClearToMark => {
                stack.clear_to_mark()?;
                collect_level -= 1;
            }
            Action::MakeArray => {
                stack.make_array()?;
                collect_level -= 1;
            }
            Action::MakeProc => {
                stack.make_proc()?;
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
