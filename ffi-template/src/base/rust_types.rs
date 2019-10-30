pub use crate::{Function, Argument};

#[derive(Default, Debug, Clone)]
pub struct ImplementBlock {
    pub self_ty: String,
    pub methods: Vec<RustFunction>
}

#[derive(Default, Debug, Clone)]
pub struct RustFunction {
    pub name: String,
    pub extern_name: String,
    pub inputs: Vec<Argument<RustTypes>>,
    pub output: Option<RustTypes>
}

#[derive(Debug, Clone)]
pub enum RustTypes {
    Ptr(String),
    Option(Box<RustTypes>),
    Result(Box<RustTypes>),
    Primitive(String),
    String,
    Json(String)
}

impl Default for RustTypes {
    fn default() -> RustTypes {
        RustTypes::Ptr("c_void".to_owned())
    }
}

impl ToString for RustTypes {
    fn to_string(&self) -> String {
        match self {
            RustTypes::Ptr(s) => format!("*mut {}", s),
            RustTypes::Option(s) => format!("Option<{}>", s.to_string()),
            RustTypes::Result(s) => format!("Result<{}>", s.to_string()),
            RustTypes::Primitive(s) => s.clone(),
            RustTypes::String => "String".to_owned(),
            RustTypes::Json(s) => format!("String /*{}*/", s),
        }
    }
}