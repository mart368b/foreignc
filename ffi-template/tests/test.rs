use ffi_template::*;
use std::path::Path;

fn initialize() -> RustContext {
    RustContext {
        funcs: vec![
            RustFunction {
                name: "ASomeFunc".to_owned(),
                extern_name: "AExternName".to_owned(),
                inputs: vec![
                    RustArgument {
                        name: "a".to_owned(),
                        ty: RustTypes::Option(Box::new(RustTypes::String))
                    },
                    RustArgument {
                        name: "b".to_owned(),
                        ty: RustTypes::Primitive("bool".to_owned())
                    }
                ],
                output: Some(RustTypes::Primitive("u32".to_owned()))
            },
            RustFunction {
                name: "BSomeFunc".to_owned(),
                extern_name: "BExternName".to_owned(),
                inputs: vec![
                    RustArgument {
                        name: "a".to_owned(),
                        ty: RustTypes::String
                    },
                    RustArgument {
                        name: "b".to_owned(),
                        ty: RustTypes::Primitive("bool".to_owned())
                    }
                ],
                output: Some(RustTypes::Primitive("u32".to_owned()))
            }
        ],
        free_funcs: Vec::new(),
        structs: Vec::new(),
    }
}

#[test]
fn build() -> TResult<()> {
    let con = initialize();
    con.generate_python_api("test.py", None)?;
    Ok(())
}