use crate::py_types::*;
use crate::*;
use foreignc_util::to_camel_case;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::write;
use std::path::Path;
use tera::{Context, Tera};

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

    fn create_context(&self, lib_name: Option<String>) -> TResult<Context> {
        // Check if any method or function returns a string
        let return_string = self
            .funcs
            .iter()
            .any(|f| f.output.as_ref().map_or(false, |t| t.is_string()))
            || self.structs.iter().any(|s| s.ty == StructTypes::Json)
            || self.structs.iter().any(|s| {
                s.methods
                    .iter()
                    .any(|f| f.output.as_ref().map_or(false, |t| t.is_string()))
            });

        // if a string is returned check if free_string is set
        if return_string {
            let has_free_string = self
                .free_funcs
                .iter()
                .any(|f| &f.func.name == "free_string")
                || self.funcs.iter().any(|f| &f.name == "free_string");

            // Throw error
            if !has_free_string {
                return Err(TemplateError::MessageErr("Returned string without methods of deletion please add a free_string function or use foreignc_derive::generate_free_methods to generate it".to_owned()));
            }
        }

        let mut structs = self.structs.clone();
        let abi: Vec<FunctionABI> = self
            .funcs
            .iter()
            .map(|a| FunctionABI::from_rust(a, &mut structs))
            .collect();

        // Currently we dont add the free function explicitly
        //let mut free_abi: Vec<FunctionABI> = self.free_funcs.iter().map(|a| FunctionABI::from_rust(&a.func, &mut structs)).collect();
        //abi.append(&mut free_abi);

        let mut added: Vec<StructureABI> = Vec::new();
        while !structs.is_empty() {
            let current = structs.pop().unwrap();
            if added.iter().any(|a| a.self_ty == current.self_ty) {
                continue;
            }
            let converted = StructureABI::from_rust(current, &mut structs);
            added.push(converted);
        }

        type Partition = (Vec<StructureABI>, Vec<StructureABI>);
        let (boxes, others): Partition =
            added.into_iter().partition(|s| s.ty == StructTypes::Boxed);
        let (json, pointers): Partition =
            others.into_iter().partition(|s| s.ty == StructTypes::Json);

        let mut ncon = Context::new();
        ncon.insert("abis", &abi);
        ncon.insert("boxes", &boxes);
        ncon.insert("jsons", &json);
        ncon.insert("pointers", &pointers);
        let final_name = if let Some(name) = lib_name {
            name.to_owned()
        } else {
            format!("{}Lib", to_camel_case(env!("CARGO_PKG_NAME").to_owned()))
        };
        ncon.insert("lib_name", &final_name);
        Ok(ncon)
    }

    pub fn generate_python_api<P>(&self, path: P, lib_name: Option<String>) -> TResult<()>
    where
        P: AsRef<OsStr> + Sized,
    {
        let context = self.create_context(lib_name)?;
        let content = Tera::one_off(&include_str!("../templates/py.jinja"), &context, false)?;
        write(Path::new(&path), content)?;
        Ok(())
    }
}
