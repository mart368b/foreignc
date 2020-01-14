use crate::*;

#[with_abi]
pub fn return_string() -> &'static str {
    "Hello World!"
}

#[with_abi]
pub fn return_number() -> u32 {
    12345
}

#[with_abi]
pub fn return_some_number() -> Option<u32> {
    Some(12345)
}

#[with_abi]
pub fn return_none_number() -> Option<u32> {
    None
}

#[with_abi]
pub fn return_ok_number() -> Result<u32, &'static str> {
    Ok(12345)
}

#[with_abi]
pub fn return_err_str() -> Result<u32, &'static str> {
    Err("Hello World!")
}

#[with_abi]
pub fn number_argument(v: u32) {
    assert_eq!(v, 12345);
}

#[with_abi]
pub fn str_argument(v: String) {
    assert_eq!(v, "Hello World!".to_owned());
}
