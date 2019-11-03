use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Json, Boxed, Debug)]
pub struct SerdeStruct {
    value: Vec<String>,
}

#[wrap_extern(SerdeStruct as Json)]
pub fn new_serde_struct() -> SerdeStruct {
    SerdeStruct {
        value: vec!["Hello".to_owned(), "World!".to_owned()],
    }
}

type JsonStruct = SerdeStruct;

#[wrap_extern(JsonStruct as Json)]
pub fn json_to_box(s: JsonStruct) -> SerdeStruct {
    s
}

#[wrap_extern(JsonStruct as Json)]
pub fn box_to_json(s: &SerdeStruct) -> &JsonStruct {
    s
}
