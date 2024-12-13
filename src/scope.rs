//  PERF: Reference variables instead of cloning
//          - safes lots of memory in case of many nested scopes

use std::collections::HashMap;

use crate::variable::Variable;

pub struct Scope {
    variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn inherit(parent: &Scope) -> Self {
        Self {
            variables: parent.variables.clone(),
        }
    }

    fn variable_set(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn variable_get(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
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

    pub fn variable_set(&mut self, name: String, variable: Variable) {
        self.peek().variable_set(name, variable);
    }

    pub fn variable_get(&self, name: &str) -> Option<&Variable> {
        for scope in self.stack.iter().rev() {
            if let Some(variable) = scope.variable_get(name) {
                return Some(variable);
            }
        }

        None
    }
}
