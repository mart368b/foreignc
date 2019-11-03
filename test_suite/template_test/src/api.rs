use crate::*;

#[derive(Boxed)]
pub struct SomeStruct{}

#[wrap_extern]
impl SomeStruct {
    pub fn new() -> SomeStruct {
        SomeStruct{}
    }
}

