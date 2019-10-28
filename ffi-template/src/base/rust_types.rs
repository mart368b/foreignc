pub use crate::{Function, Argument};

pub struct RustArgument {
    name: String,
    ty: RustTypes,
}

impl Argument<RustTypes> for RustArgument {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &RustTypes {
        &self.ty
    }
}

pub struct RustFunction {
    name: String,
    args: Vec<RustArgument>
}

impl Function<RustArgument, RustTypes> for RustFunction {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_args(&self) -> &Vec<RustArgument> {
        &self.args
    }
}

pub enum RustTypes {
    ISize(String),
    USize(String),
    Boolean,
    String(String),
    Vector(Vec<Box<RustTypes>>),
    Result(Box<RustTypes>, Box<RustTypes>),
    Option(Box<RustTypes>),
    PrimitiveObject(String),
    GenericObject(String)
}

impl ToString for RustTypes {
    fn to_string(&self) -> String {
        match self {
            RustTypes::Boolean => "bool".to_owned(),
            RustTypes::ISize(s)
            | RustTypes::USize(s)
            | RustTypes::PrimitiveObject(s)
            | RustTypes::GenericObject(s)
            | RustTypes::String(s) => s.to_owned(),
            RustTypes::Vector(inner) => {
                format!(
                    "Vec<{}>", 
                    inner
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            RustTypes::Result(left, right) => format!("Result<{}, {}>", left.to_string(), right.to_string()),
            RustTypes::Option(s) => format!("Option<{}>", s.to_string()),
        }
    }
}