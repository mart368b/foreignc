use ffi_template::*;
use std::path::Path;

fn initialize() -> RustContext {
    RustContext {
        funcs: vec![
            RustFunction {
                name: "SomeFunc".to_owned(),
                extern_name: "ExternName".to_owned(),
                inputs: vec![
                    RustArgument {
                        name: "a".to_owned(),
                        ty: RustTypes::String
                    }
                ],
                output: None
            }
        ],
        free_funcs: Vec::new(),
        structs: Vec::new(),
    }
}

#[test]
fn build() -> TResult<()> {
    let con = initialize();
    con.generate_python_api(Path::new("test.txt"))?;
    Ok(())
}