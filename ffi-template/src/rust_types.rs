use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetaType {
    Func(RustFunction),
    Struct(RustStructure),
    FreeFunc(RustFreeFunction),
}

impl From<RustFunction> for MetaType {
    fn from (s: RustFunction) -> MetaType {
        MetaType::Func(s)
    }
}

impl From<RustStructure> for MetaType {
    fn from (s: RustStructure) -> MetaType {
        MetaType::Struct(s)
    }
}

impl From<RustFreeFunction> for MetaType {
    fn from (s: RustFreeFunction) -> MetaType {
        MetaType::FreeFunc(s)
    }
}

pub type RustFunction = Function<RustArgument, RustTypes>;


#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct FreeFunction {
    pub ty: RustTypes,
    pub func: RustFunction,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Structure<Func, Arg, Ty>  {
    pub self_ty: String,
    pub methods: Vec<Func>,
    pub destructor: Option<String>
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Function<Arg, Ty> 
where
    Arg: Default, Deserialize, Serialize, Debug, Clone,
    Ty: Default, Deserialize, Serialize, Debug, Clone
{
    pub name: String,
    pub extern_name: String,
    pub inputs: Vec<Arg>,
    pub output: Option<Ty>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct RustArgument {
    pub name: String,
    pub ty: RustTypes,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RustTypes {
    Ptr(String),
    Option(Box<RustTypes>),
    Result(Box<RustTypes>),
    Primitive(String),
    String,
    Json(String),
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
