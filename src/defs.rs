use crate::value::{NamedValue, Value};

impl NamedValue for Variable {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> Value {
        self.value.clone()
    }

    fn set_value(&mut self, val: Value) {
        self.value = val;
    }
}

impl NamedValue for Parameter {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> Value {
        self.value.clone()
    }

    fn set_value(&mut self, _val: Value) {
        panic!("Cannot set value for paramter! Something has gone wrong.");
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub index: usize,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: FunctionBody,
    pub ret_val: Value,
}
