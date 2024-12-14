use crate::{lexer::Lexer, scope::ScopeStack};
use anyhow::Result;

// Interpreter used for interpreting source files
pub struct Interpreter {
    scope_stack: ScopeStack,
}

impl Interpreter {
    // Create new interpreter
    fn new() -> Self {
        Self {
            scope_stack: ScopeStack::new(),
        }
    }

    // Interpret a string of source code, returns Err if there errors occur during interpretation
    pub fn interpret(content: String) -> Result<()> {
        let inter = Self::new();

        let tokens = Lexer::tokenize(content.as_str());

        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }
}
