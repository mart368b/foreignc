use crate::*;

#[wrap_extern]
pub fn func() {}
#[wrap_extern]
pub fn func_with_param(a: bool, b: i32, c: u32) {}
#[wrap_extern]
pub fn func_with_return() -> bool {true}