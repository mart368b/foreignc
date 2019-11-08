use crate::*;
use serde::{Deserialize, Serialize};

pub struct PythonABI {
    name: String,
    args: Vec<PythonTypes>
}

pub enum PythonTypes {
    C_VOID,
    Primitive(String),
    Option(Box<PythonTypes>),
    Result(Box<PythonTypes>),
    Box,
    C_CHAR_P,
}

impl PythonTypes {
    fn from_rust<T: AsRef<RustTypes>>(ty: T) -> PythonTypes {
        match ty.as_ref() {
            RustTypes::Ptr(_) => PythonTypes::C_VOID,
            RustTypes::Option(s) => PythonTypes::Option(Box::new(PythonTypes::from_rust(s))),
            RustTypes::Result(s) => PythonTypes::Result(Box::new(PythonTypes::from_rust(s))),
            RustTypes::Primitive(s) => match s.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                | "u32" | "u64" | "u128" | "usize" | "char" => PythonTypes::Primitive("int".to_owned()),
                "f32" | "f64" => PythonTypes::Primitive("str".to_owned()),
                "bool" => PythonTypes::Primitive("bool".to_owned()),
                _ => PythonTypes::C_VOID
            },
            RustTypes::Json(_) 
            | RustTypes::String => PythonTypes::C_CHAR_P
        }
    }
}

impl ToString for PythonTypes {
    fn to_string(&self) -> String {
        match self {
            PythonTypes::C_CHAR_P => "c_char_p".to_owned(),
            PythonTypes::Box
            | PythonTypes::C_VOID => "c_void".to_owned(),
            PythonTypes::Primitive(p) => p,
            PythonTypes::Option(s) => s.to_string(),
            PythonTypes::Result(s) => s.to_string(),
        }
    }
}