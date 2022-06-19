use super::native_func::Function;
use super::NamedVal;
use super::Result;
use crate::types::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct VmContext {
    vars: HashMap<String, NamedVal>,
    funcs: HashMap<String, Function>,
}

impl VmContext {
    pub fn new(vars: HashMap<String, NamedVal>, funcs: HashMap<String, Function>) -> Self {
        VmContext { vars, funcs }
    }
}

impl Default for VmContext {
    fn default() -> Self {
        Self::new(HashMap::new(), HashMap::new())
    }
}

pub trait Contextable {
    fn get_var(&self, name: &str) -> Option<NamedVal>;
    fn get_func(&self, name: &str) -> Option<&Function>;

    fn push_var(&mut self, var: NamedVal);
    fn push_func(&mut self, func: Function);
}

pub trait ExecutionContext: Contextable {
    fn contains_var(&self, var_name: &str) -> bool;
    fn contains_func(&self, func_name: &str) -> bool;

    fn set_var(&mut self, name: &str, value: Value) -> Result<()>;
}

impl Contextable for VmContext {
    fn get_var(&self, name: &str) -> Option<NamedVal> {
        self.vars.get(name).map(Rc::clone)
    }

    fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    fn push_var(&mut self, var: NamedVal) {
        let name = { var.borrow().get_name().to_string() };
        println!("Pushing var: {} = {:?}", name, var.borrow().get_value());
        self.vars.insert(name, var);
    }

    fn push_func(&mut self, func: Function) {
        println!("Pushing func: {:?}", func);
        self.funcs.insert(func.get_name().to_owned(), func);
    }
}

impl<T: Contextable> ExecutionContext for T {
    fn contains_var(&self, var_name: &str) -> bool {
        self.get_var(var_name).is_some()
    }

    fn contains_func(&self, func_name: &str) -> bool {
        self.get_func(func_name).is_some()
    }

    // Make this an operation and move to Value?
    fn set_var(&mut self, name: &str, value: Value) -> Result<()> {
        println!("Setting var: {} = {:?}", name, value);
        let name = name;
        let var = self
            .get_var(name)
            .ok_or(format!("Could not find variable '{}'!", name))?;
        let mut var = var.borrow_mut();
        var.set_value(value);
        Ok(())
    }
}