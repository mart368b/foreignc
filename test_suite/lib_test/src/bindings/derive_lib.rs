use crate::*;

#[link(name = "derive_lib.dll")]
extern "C" {
    // lib.rs
    pub fn free_string(ptr: *mut c_char);
    pub fn last_error() -> *mut c_char;

    // base_test.rs
    pub fn hello_world() -> *mut c_char;
    pub fn add(a: i32, b: i32) -> i32;
    pub fn throw_err(is_err: bool) -> *mut c_char;
    pub fn return_option(is_opt: bool) -> *mut c_char;

    // serde_test.rs
    pub fn free_serde_struct(ptr: *mut c_void);
    pub fn new_serde_struct() -> *mut c_char;
    pub fn json_to_box(s: *mut c_char) -> *mut c_void;
    pub fn box_to_json(s: *mut c_void) -> *mut c_char;

    //impl_test.rs
    pub fn new_boxed_struct() -> *mut c_void;
    pub fn inc_boxed_struct(this: *mut c_void);
    pub fn get_boxed_struct(this: *const c_void) -> u32;
    pub fn free_boxed_struct(ptr: *mut c_void);
}
