use crate::{
    lexer::{Lexer, Token},
    scope::{Scope, ScopeStack},
};
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

    pub fn interpret(&self, content: String) -> Result<()> {
        let tokens = Lexer::tokenize(content.as_str());

        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }
}
