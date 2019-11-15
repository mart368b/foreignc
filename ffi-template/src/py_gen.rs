use crate::*;
use crate::py_types::*;
use std::ffi::OsStr;
use std::path::Path;
use tera::{Context, Tera};
use std::fs::write;
use foreignc_util::to_camel_case;
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

    fn create_context(&self, lib_name: Option<String>) -> Context {
        
        let mut structs = self.structs.clone();
        
        let mut abi: Vec<FunctionABI> = self.funcs.iter().map(|a| FunctionABI::from_rust(a, &mut structs)).collect();
        let mut free_abi: Vec<FunctionABI> = self.free_funcs.iter().map(|a| FunctionABI::from_rust(&a.func, &mut structs)).collect();
        abi.append(&mut free_abi);


        let mut added = Vec::new();
        while structs.len() != 0 {
            let first = structs.pop().unwrap();
            let converted = StructureABI::from_rust(first, &mut structs);
            added.push(converted);
        }
        
        type Partition = (Vec<StructureABI>, Vec<StructureABI>);
        let (boxes, others): Partition = added.into_iter().partition(|s| s.ty == StructTypes::Boxed);
        let (json, pointers): Partition = others.into_iter().partition(|s| s.ty == StructTypes::Json);
        
        let mut ncon = Context::new();
        ncon.insert("abis", &abi);
        ncon.insert("boxes", &boxes);
        ncon.insert("jsons", &json);
        ncon.insert("pointers", &pointers);
        let final_name = if let Some(name) = lib_name {
            name.to_owned()
        }else {
            format!("{}Lib", to_camel_case(env!("CARGO_PKG_NAME").to_owned()))
        };
        ncon.insert("lib_name", &final_name);
        ncon
    }

    pub fn generate_python_api<P>(&self, path: P, lib_name: Option<String>) -> TResult<()> 
    where
        P: AsRef<OsStr> + Sized
    {
        let context = self.create_context(lib_name);
        let content = Tera::one_off(
            &include_str!("../templates/py.jinja"), 
            &context,
            false
        )?;
        println!("{}", content);
        write(Path::new(&path), content)?;
        Ok(())
    }
}