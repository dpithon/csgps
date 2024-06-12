use clap::Parser;
use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

// TODO: complete "wrong argument types {:?} ..." messages

mod builtin;
mod dstack;
mod engine;
mod item;
mod token;
mod xstack;

use builtin::Builtin;
pub use dstack::DictStack;
use engine::Engine;
pub use item::Item;
use token::{get_action, Token};
pub use xstack::ExecStack;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

pub enum Action {
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

fn main() {
    let args = Args::parse();
    match execute(&args.filename) {
        Err(e) => println!("ERR: {e}"),
        Ok(_) => println!("bye."),
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
    let mut engine = Engine::new();

    loop {
        engine.process_execution_stack()?;

        let action = match lex.next() {
            None => break,
            Some(Err(_)) => return Err(format!("error while parsing {filename}: {}", lex.slice())),
            Some(Ok(token)) => get_action(&token),
        };

        engine.execute_action(action)?;
    }

    Ok(())
}
