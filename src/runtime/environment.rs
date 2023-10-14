use std::collections::{HashMap, HashSet};

use super::values::RuntimeValue;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub variables: HashMap<String, RuntimeValue>,
    pub constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        Self {
            parent: match parent {
                Some(parent) => Some(Box::new(parent)),
                None => None,
            },
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn declare_variable(
        &mut self,
        variable_name: String,
        value: RuntimeValue,
        constant: bool,
    ) -> RuntimeValue {
        if self.variables.contains_key(&variable_name) {
            panic!("Variable {} already declared", variable_name);
        }

        self.variables.insert(variable_name.clone(), value.clone());

        if constant {
            self.constants.insert(variable_name.clone());
        }

        value
    }

    pub fn assign_variable(&mut self, variable_name: String, value: RuntimeValue) -> RuntimeValue {
        let mut environment = self.resolve(variable_name.clone());

        if environment.constants.contains(&variable_name) {
            panic!("Cannot assign to constant variable {}", variable_name);
        }

        environment.variables.insert(variable_name, value.clone());

        value
    }

    pub fn peek_variable(&self, variable_name: String) -> RuntimeValue {
        let environment = self.resolve(variable_name.clone());
        environment.variables.get(&variable_name).unwrap().clone()
    }

    pub fn resolve(&self, variable_name: String) -> Environment {
        if self.variables.contains_key(&variable_name) {
            return self.clone();
        }

        match self.parent {
            Some(ref parent) => parent.resolve(variable_name),
            None => panic!("Variable {} not found", variable_name),
        };

        todo!();
    }
}