use std::env;
use std::fs::read;
use std::process;

use interpreter::Interpreter;

mod interpreter;
mod lexer;
mod scope;
mod variable;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let content = match read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error converting file content to string: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error reading file {}: {}", path, e);
            process::exit(1);
        }
    };

    Interpreter::interpret(content).unwrap();
}
