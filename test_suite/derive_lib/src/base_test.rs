use crate::*;

#[with_abi]
pub fn hello_world() -> &'static str {
    "Hello World!"
}

#[with_abi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[with_abi]
pub fn return_option(is_opt: bool) -> Option<String> {
    if is_opt {
        Some("Some".to_owned())
    } else {
        None
    }
}
