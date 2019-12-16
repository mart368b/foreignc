use crate::*;
use derive_lib::{JsonStruct, BoxedStruct};
use foreignc::CResult;

#[link(name = "derive_lib.dll")]
extern "C" {
    // lib.rs
    pub fn free_string(ptr: *mut c_char);
    pub fn free_coption(ptr: *mut c_void);
    pub fn free_cresult(ptr: *mut c_void);

    // base_test.rs
    pub fn return_string_ffi() -> *mut CResult<&'static str, String>;
    pub fn return_number_ffi() -> *mut CResult<u32, String>;
    pub fn return_some_number_ffi() -> *mut CResult<Option<u32>, String>;
    pub fn return_none_number_ffi() -> *mut CResult<Option<u32>, String>;
    pub fn return_ok_number_ffi() -> *mut CResult<CResult<u32, &'static str>, String>;
    pub fn return_err_str_ffi() -> *mut CResult<CResult<u32, &'static str>, String>;
    pub fn number_argument_ffi(v: u32) -> *mut CResult<(), String>;
    pub fn str_argument_ffi(v: *mut c_char) -> *mut CResult<(), String>;

    // serde_test.rs
    pub fn inc_json_struct(this: *mut c_void) -> *mut CResult<*mut JsonStruct, String>;
    pub fn get_json_struct(this: *const c_void) -> *mut CResult<u32, String>;
    pub fn new_json_struct() -> *mut CResult<*mut JsonStruct, String>;

    //impl_test.rs
    pub fn new_boxed_struct() -> *mut CResult<*mut BoxedStruct, String>;
    pub fn inc_boxed_struct(this: *mut c_void) -> *mut CResult<(), String>;
    pub fn get_boxed_struct(this: *const c_void) -> *mut CResult<u32, String>;
    pub fn free_boxed_struct(ptr: *mut c_void) -> *mut CResult<(), String>;
}
