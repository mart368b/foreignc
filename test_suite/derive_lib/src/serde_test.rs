use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Json, Debug)]
pub struct SerdeStruct {
    value: Vec<String>,
}

#[with_abi]
pub fn new_serde_struct() -> SerdeStruct {
    SerdeStruct {
        value: vec!["Hello".to_owned(), "World!".to_owned()],
    }
}
