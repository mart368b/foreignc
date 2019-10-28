#[derive(Default, Debug)]
pub struct ABI {
    methods: Vec<Function>,
    implements: Vec<Implement>
}

#[derive(Default, Debug)]
pub struct Implement {
    pub self_ty: String,
    pub methods: Vec<Function>
}

#[derive(Default, Debug)]
pub struct Function {
    pub name: String,
    pub extern_name: String,
    pub inputs: Vec<(String, Arg)>,
    pub output: Option<Arg>
}

#[derive(Debug)]
pub enum Arg {
    Ptr(String),
    Option(Box<Arg>),
    Result(Box<Arg>),
    Pimitive(String),
    String,
    JSON(String)
}