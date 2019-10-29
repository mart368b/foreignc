use crate::*;

#[derive(Boxed)]
pub struct BoxedStruct {
    acc: u32
}

#[wrap_extern]
impl BoxedStruct {
    pub fn new() -> BoxedStruct {
        BoxedStruct {
            acc: 0
        }
    }

    pub fn inc(&mut self) {
        self.acc += 1;
    }

    pub fn get(&self) -> u32 {
        self.acc
    }
}