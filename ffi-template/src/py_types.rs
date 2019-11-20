use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructureABI {
    pub self_ty: String,
    pub methods: Vec<FunctionABI>,
    pub destructor: Option<String>,
    pub ty: StructTypes
}
impl StructureABI {
    pub fn from_rust(s: RustStructure, structs: &mut Vec<RustStructure>) -> StructureABI {
        StructureABI {
            self_ty: s.self_ty,
            methods: s.methods.iter().map(|f| FunctionABI::from_rust(f, structs)).collect(),
            destructor: s.destructor,
            ty: s.ty
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionABI {
    extern_name: String,
    extern_inputs: Vec<String>,
    extern_output: Option<String>,

    arg_list: Vec<String>,
    is_method: bool,

    sig_name: String,
    sig_inputs: Vec<String>,
    sig_output: Option<String>,
}

impl FunctionABI {
    pub fn from_rust<T: AsRef<RustFunction>>(func: T, structs: &mut Vec<RustStructure>) -> FunctionABI {
        let f = func.as_ref();

        let py_input: Vec<PythonTypes> = f.inputs.iter().map(|a| PythonTypes::from_rust(&a.ty, structs)).collect();
        let py_output: Option<PythonTypes> = f.output.clone().map(|a| PythonTypes::from_rust(a, structs));

        FunctionABI{
            extern_name: f.extern_name.to_owned(),
            extern_inputs: py_input.iter().map(|input| input.get_extern()).collect(),
            extern_output: py_output.clone().map(|output| output.get_extern()),
            
            arg_list: f.inputs.iter().map(|arg| arg.name.replace("this", "self")).collect(),
            is_method: f.inputs.iter().any(|arg| arg.name == "this"),

            sig_name: f.name.to_owned(),
            sig_inputs: f.inputs
                .iter()
                .zip(py_input.iter())
                .filter(|(arg, _)| &arg.name != "this")
                .map(|(arg, input)| format!("{}: {}", arg.name, input.get_hint()))
                .collect(),
            sig_output: py_output.clone().map(|output| output.get_hint()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PythonTypes {
    Pointer(String),
    Primitive(String),
    Option(Box<PythonTypes>),
    Result(Box<PythonTypes>, Box<PythonTypes>),
    String,
}

impl PythonTypes {
    pub fn from_rust<T: AsRef<RustTypes>>(ty: T, structs: &mut Vec<RustStructure>) -> PythonTypes {
        match ty.as_ref() {
            RustTypes::Ptr(s) => {
                if structs.iter().find(|st| &st.self_ty == s).is_none() {
                    structs.push(RustStructure {
                        self_ty: s.to_owned(),
                        methods: Vec::new(),
                        destructor: None,
                        ty: StructTypes::RawPointer
                    })
                }
                PythonTypes::Pointer(s.to_owned())
            },
            RustTypes::Option(s) => PythonTypes::Option(Box::new(PythonTypes::from_rust(s, structs))),
            RustTypes::Result(ok, err) => PythonTypes::Result(Box::new(PythonTypes::from_rust(ok, structs)), Box::new(PythonTypes::from_rust(err, structs))),
            RustTypes::Primitive(s) => match s.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                | "u32" | "u64" | "u128" | "usize" | "char" => PythonTypes::Primitive("int".to_owned()),
                "f32" | "f64" => PythonTypes::Primitive("float".to_owned()),
                "bool" => PythonTypes::Primitive("bool".to_owned()),
                _ => unimplemented!()
            },
            RustTypes::String => PythonTypes::String,
        }
    }

    fn get_hint(&self) -> String {
        match self {
            PythonTypes::String => "str".to_owned(),
            PythonTypes::Pointer(s) => s.to_owned(),
            PythonTypes::Primitive(p) => p.to_owned(),
            PythonTypes::Option(s) => format!("Option[{}]", s.get_hint()),
            PythonTypes::Result(ok, err) => format!("Result[{}, {}]", ok.get_hint(), err.get_hint()),
        }
    }

    fn get_extern(&self) -> String {
        match self {
            PythonTypes::String => "LibString".to_owned(),
            PythonTypes::Pointer(s) => s.to_owned(),
            PythonTypes::Primitive(p) => p.to_owned(),
            PythonTypes::Option(s) => format!("Option[{}]", s.get_extern()),
            PythonTypes::Result(ok, err) => format!("Result[{}, {}]", ok.get_extern(), err.get_extern()),
        }
    }
}