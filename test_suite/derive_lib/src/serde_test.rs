use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Json, Deserialize, Serialize)]
pub struct JsonStruct {
    acc: u32,
}

#[with_abi]
impl JsonStruct {
    pub fn new() -> JsonStruct {
        JsonStruct { acc: 0 }
    }

    pub fn inc(mut self) -> JsonStruct {
        self.acc += 1;
        self
    }

    pub fn get(self) -> u32 {
        self.acc
    }
}
