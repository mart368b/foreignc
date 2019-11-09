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
        let abi: Vec<PythonABI> = self.funcs.iter().map(|a| PythonABI::from_rust(a, self)).collect();
        ncon.insert("abis", &abi);
        ncon.insert("free_funcs", &self.free_funcs);
        ncon
    }

    

    fn create_tera(&self) -> TResult<Tera> {
        let mut t = Tera::parse("templates/py/*")?;
        t.build_inheritance_chains()?;
        Ok(t)
    }

    pub fn generate_python_api(&self, path: &Path) -> TResult<()> {
        let tmpl = self.create_tera()?;
        let context = self.create_context();
        let content = tmpl.render("index.jinja", &context)?;
        println!("{}", &content);
        write(path, content)?;
        Ok(())
    }
}