use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::helpers::{
    build_bool_runtime_value::build_bool_runtime_value,
    build_native_function::build_native_function,
    build_null_runtime_value::build_null_runtime_value,
};

use super::{
    functions::{get_time, print},
    values::RuntimeValue,
};

pub fn build_scope(env: &mut Environment) {
    // Global variables
    env.declare_variable("true".to_string(), build_bool_runtime_value(true), true);
    env.declare_variable("false".to_string(), build_bool_runtime_value(false), true);
    env.declare_variable("null".to_string(), build_null_runtime_value(), true);

    // Global functions
    env.declare_variable("print".to_string(), build_native_function(print), true);
    env.declare_variable(
        "get_current_time".to_string(),
        build_native_function(get_time),
        true,
    );
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<ScopeType>>,
    pub variables: HashMap<String, RuntimeValue>,
    pub constants: HashSet<String>,
}

#[derive(Debug, Clone)]
pub enum ScopeType {
    Global(Environment),
    Local(Rc<RefCell<Environment>>),
}

impl Environment {
    pub fn new(parent: Option<ScopeType>) -> Self {
        let is_global = match &parent {
            Some(_) => false,
            None => true,
        };

        let mut env = Environment {
            parent: match parent {
                Some(parent) => Some(Box::new(parent)),
                None => None,
            },
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

        if is_global {
            build_scope(&mut env);
        }

        env
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

        match &self.parent {
            Some(parent) => match parent.as_ref() {
                ScopeType::Global(parent) => parent.resolve(variable_name),
                ScopeType::Local(parent) => parent.borrow().resolve(variable_name),
            },
            None => panic!("Variable {} not found", variable_name),
        };

        todo!();
    }
}
