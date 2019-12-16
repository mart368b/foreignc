pub use foreignc::*;
use serde::{Deserialize, Serialize};

generate_free_methods!();

#[derive(Boxed, Serialize, Deserialize, Debug)]
pub struct BoxedStruct{
    name: String,
    value: String
}

#[derive(Json, Serialize, Deserialize, Debug)]
pub struct JsonStruct{
    name: String,
    value: String
}

#[derive(Debug)]
pub struct UnknownStruct{}

#[with_abi]
impl JsonStruct {
    pub fn new() -> JsonStruct {
        JsonStruct {
            name: "Hello".to_owned(),
            value: "World!".to_owned()
        }
    }

    pub fn debug(self) {
        println!("debug: {:?}", self);
    }
}

#[with_abi]
impl BoxedStruct {
    pub fn new() -> BoxedStruct {
        BoxedStruct {
            name: "Boxed".to_owned(),
            value: "World!".to_owned()
        }
    }

    pub fn debug(&self) {
        println!("debug: {:?}", self);
    }
}

#[with_abi]
pub fn get_unknown() -> *mut UnknownStruct {
   Box::into_raw(Box::new(UnknownStruct{}))
}

#[with_abi]
pub fn get_string() -> &'static str {
    "ABCDEFGHIJKLM"
}

#[with_abi]
pub fn get_some() -> Option<u64> {
    Some(12345)
}

#[with_abi]
pub fn get_none() -> Option<u64> {
    None
}

#[with_abi]
pub fn set_option(v: Option<u32>) {
    println!("Recieved optional number: {:?}", v)
}

#[with_abi]
pub fn get_ok() -> Result<&'static str, &'static str> {
    Ok("This is ok")
}

#[with_abi]
pub fn get_err() -> Result<&'static str, &'static str> {
    Err("This is bad")
}

#[with_abi]
pub fn get_nested() -> Option<Option<Option<&'static str>>> {
    Some(Some(Some("Hello World!")))
}

#[with_abi]
pub fn set_nested(v: Option<Option<u32>>) {
    println!("Recieved nested optional number: {:?}", v);
}

#[with_abi]
pub fn get_nested_combined() -> Option<Option<Result<Option<&'static str>, &'static str>>> {
    Some(Some(Ok(Some("Hello World!"))))
}