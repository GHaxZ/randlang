use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use anyhow::{anyhow, Result};

use crate::variable::Variable;

#[derive(Default)]
pub struct Scope {
    variables: HashMap<String, Rc<RefCell<Variable>>>,
    owned_variables: Vec<String>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    fn inherit(parent: &Scope) -> Self {
        Self {
            variables: parent.variables.clone(),
            owned_variables: Vec::new(),
        }
    }

    fn variable_declare(&mut self, name: &str, variable: Variable) {
        self.owned_variables.push(name.to_string());
        self.variables
            .insert(name.to_string(), Rc::new(RefCell::new(variable)));
    }

    fn variable_set(&mut self, name: &str, variable: Variable) -> Result<()> {
        match self.variables.get(name) {
            Some(existing_var) => {
                *existing_var.borrow_mut() = variable;
                Ok(())
            }
            None => Err(anyhow!("Unknown variable identifier \"{}\"", name)),
        }
    }

    fn variable_get(&self, name: &str) -> Option<Ref<Variable>> {
        Some(self.variables.get(name)?.borrow())
    }
}

pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Scope::new()],
        }
    }

    pub fn peek(&mut self) -> Option<&mut Scope> {
        self.stack.last_mut()
    }

    pub fn pop(&mut self) -> Option<Scope> {
        let mut scope = self.stack.pop()?;

        for var_name in scope.owned_variables.drain(..) {
            scope.variables.remove(&var_name);
        }

        Some(scope)
    }

    pub fn push(&mut self) {
        let new_scope = match self.stack.last() {
            Some(s) => Scope::inherit(s),
            None => Scope::new(),
        };

        self.stack.push(new_scope);
    }

    pub fn variable_get(&self, name: &str) -> Option<Ref<Variable>> {
        self.stack.last()?.variable_get(name)
    }

    pub fn variable_declare(&mut self, name: &str, variable: Variable) {
        if let Some(scope) = self.stack.last_mut() {
            scope.variable_declare(name, variable)
        }
    }

    pub fn variable_set(&mut self, name: &str, variable: Variable) -> Result<()> {
        self.stack
            .last_mut()
            .expect("No scopes on stack, this should never happen, please report")
            .variable_set(name, variable)
    }
}
