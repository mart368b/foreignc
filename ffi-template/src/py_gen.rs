use crate::*;
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

    fn convert_ty(ty: RustTypes) -> String {
        match ty {
            RustTypes::Ptr(s) => s.to_owned(),
            RustTypes::Option(s) => RustContext::convert_ty(*s.clone()),
            RustTypes::Result(s) => RustContext::convert_ty(*s.clone()),
            RustTypes::Primitive(s) => s.to_owned(),
            RustTypes::Json(_)
            | RustTypes::String => "c_char_p".to_owned(),
        }
    }

    fn create_context(&self) -> Context {
        let mut ncon = Context::new();
        ncon.insert("structs", &self.structs);
        ncon.insert("funcs", &self.funcs);
        ncon.insert("free_funcs", &self.free_funcs);
        let class_name = format!("{}Lib", std::env::var("CARGO_PKG_NAME").unwrap());
        ncon.insert("class_name", &class_name);
        ncon.insert("convert_ty", &RustContext::convert_ty);
        ncon
    }

    pub fn generate_python_api(&self, path: &Path) -> TResult<()> {
        let mut tmpl = Tera::parse("templates/py/*")?;
        tmpl.build_inheritance_chains()?;
        let context = self.create_context();
        let content = tmpl.render("index.jinja", &context)?;
        println!("{}", &content);
        write(path, content)?;
        Ok(())
    }
}