use csgps::Scanner;
use std::env;

use log::debug;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut interactive = false;
    let mut scanner = Scanner::new();

    for filename in args[1..].iter() {
        if filename == "-i" {
            debug!("found flag interactive mode");
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
