pub use foreignc::*;
use serde::{Deserialize, Serialize};

generate_free_string!();

#[derive(Boxed)]
pub struct BoxedStruct{}

#[derive(Json, Serialize, Deserialize)]
pub struct JsonStruct{}

#[wrap_extern]
pub fn get_string() -> &'static str {
    "Hello World!"
}

#[wrap_extern]
pub fn parse_string(s: String) {
    println!("a + {}", s);
}