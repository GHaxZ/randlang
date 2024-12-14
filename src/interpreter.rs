use crate::{lexer::Lexer, scope::ScopeStack};
use anyhow::Result;

pub struct Interpreter {
    scope_stack: ScopeStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope_stack: ScopeStack::new(),
        }
    }

    pub fn interpret(content: String) -> Result<()> {
        let inter = Self::new();

        let tokens = Lexer::tokenize(content.as_str());

        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }
}
