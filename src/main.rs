use csgsl::Engine;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interactive = false;
    let mut engine = Engine::new();

    for filename in args[1..].iter() {
        if filename == "-i" {
            interactive = true;
            continue;
        }

        if let Err(e) = engine.execute_file(filename) {
            println!("Error in {filename}: {e}");
            return;
        }
    }

    if interactive {
        engine.enter_repl();
    }
    println!("bye.");
}
