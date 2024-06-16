use crate::Engine;
use crate::ProcBuilder;
use crate::Token;
use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

pub struct Scanner {
    proc_builder: ProcBuilder,
    engine: Engine,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            proc_builder: ProcBuilder::new(),
            engine: Engine::new(),
        }
    }
}

impl Scanner {
    pub fn new() -> Self {
        Scanner::default()
    }

    pub fn execute_string(&mut self, contents: &str) -> Result<(), String> {
        let mut lex = Token::lexer(contents);

        loop {
            self.engine.process_execution_stack()?;

            match lex.next() {
                Some(Ok(Token::BeginProc)) => self.proc_builder.open(),
                Some(Ok(Token::EndProc)) => {
                    if !self.proc_builder.is_open() {
                        return Err("syntax error".to_string());
                    }
                    if let Some(proc) = self.proc_builder.close() {
                        self.engine.push(proc);
                    }
                }
                Some(Ok(Token::ImmName(name))) => {
                    let response = self.engine.get_object_by_name(&name);
                    match response {
                        Some(object) => {
                            if self.proc_builder.is_open() {
                                self.proc_builder.push(object);
                            } else {
                                self.engine.push(object);
                            }
                        }
                        None => {
                            todo!("undefined error")
                        }
                    }
                }
                Some(Ok(token)) => {
                    let object = token.to_object();
                    if self.proc_builder.is_open() {
                        self.proc_builder.push(object);
                    } else {
                        self.engine.process_object(object)?;
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
            let readline = rl.readline(&format!("csg-PS [{}] > ", self.engine.get_stack_size()));
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
}
