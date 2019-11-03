use crate::*;

#[wrap_extern]
pub fn hello_world() -> &'static str {
    "Hello World!"
}

#[wrap_extern]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wrap_extern]
pub fn throw_err(is_err: bool) -> Result<String, SimpleError> {
    if is_err {
        Err(SimpleError::new("Err"))
    } else {
        Ok("Ok".to_owned())
    }
}

#[wrap_extern]
pub fn return_option(is_opt: bool) -> Option<String> {
    if is_opt {
        Some("Some".to_owned())
    } else {
        None
    }
}
