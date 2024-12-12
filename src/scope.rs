use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::variable::Variable;

pub struct Scope {
    variables: HashMap<String, Rc<RefCell<Variable>>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn inherit(parent: &Scope) -> Self {
        Self {
            variables: parent.variables.clone(),
        }
    }

    pub fn variable_set(&mut self, name: String, variable: Rc<RefCell<Variable>>) {
        self.variables.insert(name, variable);
    }

    pub fn variable_get(&self, name: &str) -> Option<Variable> {
        self.variables.get(name).map(|rc| rc.borrow().clone())
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

    pub fn peek(&mut self) -> &mut Scope {
        self.stack.last_mut().unwrap()
    }

    pub fn pop(&mut self) -> Option<Scope> {
        self.stack.pop()
    }

    pub fn push(&mut self) {
        let new_scope = Scope::inherit(self.stack.last().unwrap());
        self.stack.push(new_scope);
    }
}
