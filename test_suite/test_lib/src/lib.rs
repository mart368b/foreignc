mod simple_error;
use simple_error::SimpleError;

use std::error::Error;

use foreignc::*;

generate_free_string!();
generate_last_error!();

#[wrap_extern]
pub fn hello_world() -> &'static str{
    "Hello World!"
}

#[wrap_extern]
pub fn add(a: i32, b:i32) -> i32 {
    a + b
}

#[wrap_extern]
pub fn throw_err(is_err: bool) -> Result<String, SimpleError> {
    if is_err {
        Err(SimpleError::new("Err"))
    }else {
        Ok("Ok".to_owned())
    }
}