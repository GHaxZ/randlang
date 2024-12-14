use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use anyhow::{anyhow, Result};

use crate::variable::Variable;

// Scope used for keeping track of things inside a code scope
pub struct Scope {
    // All variables available to this scope
    variables: HashMap<String, Rc<RefCell<Variable>>>,

    // All variables owned by this scope
    owned_variables: Vec<String>,
}

impl Scope {
    // Create a new scope
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            owned_variables: Vec::new(),
        }
    }

    // Create a new scope, inheriting from a parent scope
    fn inherit(parent: &Scope) -> Self {
        Self {
            variables: parent.variables.clone(),
            owned_variables: Vec::new(),
        }
    }

    // Declare a new variable in this scope, allows variable shadowing from parent scopes
    fn variable_declare(&mut self, name: &str, variable: Variable) {
        self.owned_variables.push(name.to_string());
        self.variables
            .insert(name.to_string(), Rc::new(RefCell::new(variable)));
    }

    // Set the value of an already declared variable, returns Err in case no declared variable with
    // the specified name exists
    //  TODO: Figure out what to do about assigning different variable types
    fn variable_set(&mut self, name: &str, variable: Variable) -> Result<()> {
        match self.variables.get(name) {
            Some(existing_var) => {
                *existing_var.borrow_mut() = variable;
                Ok(())
            }
            None => Err(anyhow!("Unknown variable identifier \"{}\"", name)),
        }
    }

    // Get a reference to a variable by name inside this scope, None if no variable is found with the specified name
    //  NOTE: Maybe return a clone of current variable state instead of reference?
    fn variable_get(&self, name: &str) -> Option<Ref<Variable>> {
        Some(self.variables.get(name)?.borrow())
    }
}

pub struct ScopeStack {
    stack: Vec<Scope>,
}

// Stack of Scopes, where Scopes can be pushed and popped, inherit from each other.
// Allows easy access to current scope information.
impl ScopeStack {
    // Create new ScopeStack with base scope
    pub fn new() -> Self {
        Self {
            stack: vec![Scope::new()],
        }
    }

    // Get current scope without removing it
    pub fn peek(&mut self) -> Option<&mut Scope> {
        self.stack.last_mut()
    }

    // Get current scope and remove it
    pub fn pop(&mut self) -> Option<Scope> {
        let mut scope = self.stack.pop()?;

        for var_name in scope.owned_variables.drain(..) {
            scope.variables.remove(&var_name);
        }

        Some(scope)
    }

    // Push a new scope onto the stack, inheriting from the last one if possible
    pub fn push(&mut self) {
        let new_scope = match self.stack.last() {
            Some(s) => Scope::inherit(s),
            None => Scope::new(),
        };

        self.stack.push(new_scope);
    }

    // Get a variable by name inside the current scope, None if no variable is found with the specified name
    pub fn variable_get(&self, name: &str) -> Option<Ref<Variable>> {
        self.stack.last()?.variable_get(name)
    }

    // Declare a new variable within the current scope, allows variable shadowing from outer scopes
    pub fn variable_declare(&mut self, name: &str, variable: Variable) {
        if let Some(scope) = self.stack.last_mut() {
            scope.variable_declare(name, variable)
        }
    }

    // Set the value of an already declared variable, returns Err in case no declared variable with
    // the specified name exists
    pub fn variable_set(&mut self, name: &str, variable: Variable) -> Result<()> {
        self.stack
            .last_mut()
            .expect("No scopes on stack, this should never happen, please report")
            .variable_set(name, variable)
    }
}
