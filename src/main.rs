use logos::Logos;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use csgsl::Engine;
use csgsl::{get_action, Token};

fn main() {
    let args: Vec<String> = env::args().collect();
    let interactive = args.len() == 2 && args[1] == "-i";
    let mut engine = Engine::new();

    for filename in args[1..].iter() {
        if filename == "-i" {
            continue;
        }

        if let Err(e) = execute_file(&mut engine, filename) {
            println!("Error in {filename}: {e}");
            return;
        }
    }

    if interactive {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        loop {
            let readline = rl.readline(&format!("csg-PS [{}]> ", engine.get_stack_size()));
            match readline {
                Ok(line) => {
                    if let Err(e) = execute_string(&mut engine, &line) {
                        println!("Error : {e}");
                    }
                }
                Err(_) => break,
            };
        }
    }
    println!("bye.");
}

fn execute_file(engine: &mut Engine, filename: &str) -> Result<(), String> {
    let mut file = match File::open(filename) {
        Err(e) => return Err(format!("error on opening {filename}: {e}")),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("error on loading {filename}: {e}"));
    }

    execute_string(engine, &contents)
}

fn execute_string(engine: &mut Engine, contents: &str) -> Result<(), String> {
    let mut lex = Token::lexer(contents);

    loop {
        engine.process_execution_stack()?;

        match lex.next() {
            Some(Ok(token)) => engine.execute_action(get_action(&token))?,
            Some(Err(_)) => return Err(format!("parse error: {}", lex.slice())),
            None => return Ok(()),
        };
    }
}
