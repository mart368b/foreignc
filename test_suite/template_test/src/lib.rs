pub use foreignc::*;
use std::panic;
use serde::{Deserialize, Serialize};

generate_free_string!();

#[derive(Boxed, Serialize, Deserialize, Debug)]
pub struct BoxedStruct{
    name: String,
    value: String
}

impl Drop for BoxedStruct {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self);
    }
}

#[derive(Json, Serialize, Deserialize, Debug)]
pub struct JsonStruct{
    name: String,
    value: String
}

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
pub fn does_panic() -> &'static str {
    ""
}

#[with_abi]
pub fn get_string() -> &'static str {
    "Hello World!"
}

#[with_abi]
pub fn parse_string(s: String) {
    println!("a + {}", s);
}

#[with_abi]
pub fn get_number() -> u32 {
    123456
}

#[with_abi]
pub fn get_none() -> Option<u32> {
    None
}

#[with_abi]
pub fn get_some() -> Option<String> {
   Some("Some(123456)".to_owned())
}

#[with_abi]
pub fn set_some(v: Option<Option<String>>) {
    println!("---{:?}", v);
}
