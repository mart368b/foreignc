use crate::*;

#[wrap_extern]
pub fn func() {}
#[wrap_extern]
pub fn func_with_param(_a: bool, _b: i32, _c: u32) {}
#[wrap_extern]
pub fn func_with_return() -> bool {
    true
}
