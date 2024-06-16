use csgsl::Scanner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interactive = false;
    let mut scanner = Scanner::new();

    for filename in args[1..].iter() {
        if filename == "-i" {
            interactive = true;
            continue;
        }

        if let Err(e) = scanner.execute_file(filename) {
            println!("Error in {filename}: {e}");
            return;
        }
    }

    if interactive {
        scanner.enter_repl();
    }
    println!("bye.");
}
