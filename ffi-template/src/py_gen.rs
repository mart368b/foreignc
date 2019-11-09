use crate::*;
use crate::py_types::*;
use std::path::Path;
use tera::{Context, Tera};
use std::fs::write;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RustContext {
    pub structs: Vec<RustStructure>,
    pub funcs: Vec<RustFunction>,
    pub free_funcs: Vec<RustFreeFunction>,
}

impl RustContext {
    pub fn new() -> RustContext {
        RustContext::default()
    }

    fn create_context(&self) -> Context {
        let mut ncon = Context::new();
        ncon.insert("class_name", &format!("{}Lib", env!("CARGO_PKG_NAME")));
        ncon.insert("structs", &self.structs);
        ncon.insert("funcs", &self.funcs);
        let mut abi: Vec<PythonABI> = self.funcs.iter().map(|a| PythonABI::from_rust(a, self)).collect();
        let mut free_abi: Vec<PythonABI> = self.free_funcs.iter().map(|a| PythonABI::from_rust(&a.func, self)).collect();
        abi.append(&mut free_abi);
        ncon.insert("abis", &abi);
        ncon.insert("free_funcs", &self.free_funcs);
        ncon
    }

    pub fn generate_python_api(&self, path: &Path) -> TResult<()> {
        println!("{:#?}", self);
        let context = self.create_context();
        let content = Tera::one_off(
            &include_str!("../templates/py.jinja"), 
            &context,
            false
        )?;
        println!("{}", &content);
        write(path, content)?;
        Ok(())
    }
}