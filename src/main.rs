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

    // Make sure there's a file path argument
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let path = &args[1]; // Get the file path (first argument after the program name)

    // Read the content of the file
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

    // Create a new interpreter
    let mut inter = Interpreter::new(); // Ensure Interpreter has a `new` method

    // Interpret the file content
    if let Err(e) = inter.interpret(content) {
        eprintln!("Error interpreting the content: {}", e);
        process::exit(1);
    }
}
