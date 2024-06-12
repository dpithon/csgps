use logos::Logos;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use csgsl::Engine;
use csgsl::{get_action, Token};

fn main() {
    let args: Vec<String> = env::args().collect();
    for filename in args[1..].iter() {
        if let Err(e) = execute_file(filename) {
            println!("error in {filename}: {e}");
            break;
        }
    }
    println!("bye.");
}

fn execute_file(filename: &str) -> Result<(), String> {
    let mut file = match File::open(filename) {
        Err(e) => return Err(format!("error on opening {filename}: {e}")),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("error on loading {filename}: {e}"));
    }

    execute_string(&contents)
}

fn execute_string(contents: &str) -> Result<(), String> {
    let mut lex = Token::lexer(contents);
    let mut engine = Engine::new();

    loop {
        engine.process_execution_stack()?;

        match lex.next() {
            Some(Ok(token)) => engine.execute_action(get_action(&token))?,
            Some(Err(_)) => return Err(format!("parse error: {}", lex.slice())),
            None => return Ok(()),
        };
    }
}
