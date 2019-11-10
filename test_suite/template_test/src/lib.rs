pub use foreignc::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt::Debug;
use std::ptr::null_mut;

generate_free_string!();

#[derive(Boxed, Debug)]
pub struct BoxedStruct{
    name: String,
    value: String
}

#[derive(Json, Serialize, Deserialize, Debug)]
pub struct JsonStruct{
    name: String,
    value: String
}

#[wrap_extern]
pub fn get_string() -> &'static str {
    "Hello World!"
}

#[wrap_extern]
pub fn parse_string(s: String) {
    println!("a + {}", s);
}

#[wrap_extern]
pub fn get_number() -> u32 {
    123456
}

#[wrap_extern]
pub fn get_boxed_struct() -> BoxedStruct {
    BoxedStruct {
        name: "Boxed".to_owned(),
        value: "World!".to_owned()
    }
}

#[wrap_extern]
pub fn debug_box(b: &BoxedStruct) {
    println!("debug: {:?}", b);
}

#[wrap_extern]
pub fn get_json_struct() -> JsonStruct {
    JsonStruct {
        name: "Hello".to_owned(),
        value: "World!".to_owned()
    }
}

#[wrap_extern]
pub fn debug_json(b: JsonStruct) {
    println!("debug: {:?}", b);
}

#[wrap_extern]
pub fn get_none() -> Option<u32> {
    None
}

#[wrap_extern]
pub fn get_some() -> Option<JsonStruct> {
    Some(JsonStruct {
        name: "Hello".to_owned(),
        value: "World!".to_owned()
    })
}