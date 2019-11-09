use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PythonABI {
    name: String,
    inputs: Vec<String>,
    output: Option<String>,
}

impl PythonABI {
    pub fn from_rust<T: AsRef<RustFunction>>(func: T, con: &RustContext) -> PythonABI {
        let f = func.as_ref();

        PythonABI{
            name: f.extern_name.to_owned(),
            inputs: f.inputs.iter().map(|a| PythonTypes::from_rust(&a.ty).get_input_handler()).collect(),
            output: f.output.clone().map(|a| PythonTypes::from_rust(a).get_output_handler()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum PythonTypes {
    Pointer(String),
    Primitive(String),
    Option(Box<PythonTypes>),
    Result(Box<PythonTypes>),
    CCHARP,
}

impl PythonTypes {
    pub fn from_rust<T: AsRef<RustTypes>>(ty: T) -> PythonTypes {
        match ty.as_ref() {
            RustTypes::Ptr(s) => PythonTypes::Pointer(s.to_owned()),
            RustTypes::Option(s) => PythonTypes::Option(Box::new(PythonTypes::from_rust(s))),
            RustTypes::Result(s) => PythonTypes::Result(Box::new(PythonTypes::from_rust(s))),
            RustTypes::Primitive(s) => match s.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                | "u32" | "u64" | "u128" | "usize" | "char" => PythonTypes::Primitive("int".to_owned()),
                "f32" | "f64" => PythonTypes::Primitive("float".to_owned()),
                "bool" => PythonTypes::Primitive("bool".to_owned()),
                _ => unimplemented!()
            },
            RustTypes::String => PythonTypes::CCHARP,
        }
    }

    fn get_input_handler(&self) -> String {
        match self {
            PythonTypes::CCHARP => "c_char_p".to_owned(),
            PythonTypes::Pointer(s) => s.to_owned(),
            PythonTypes::Primitive(p) => p.to_owned(),
            PythonTypes::Option(s) => format!("handle_input_option({})", s.get_input_handler()),
            PythonTypes::Result(s) => s.get_input_handler(),
        }
    }

    fn get_output_handler(&self) -> String {
        match self {
            PythonTypes::CCHARP => "lib_char_p('--class_name--')".to_owned(),
            PythonTypes::Pointer(s) => s.to_owned(),
            PythonTypes::Primitive(p) => p.to_owned(),
            PythonTypes::Option(s) => format!("handle_output_option({})", s.get_output_handler()),
            PythonTypes::Result(s) => format!("handle_result({})", s.get_output_handler()),
        }
    }
}