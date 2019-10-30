mod bindings;
mod tests;

pub use bindings::*;

pub use std::os::raw::{c_char, c_void};
pub use std::ffi::CStr;

pub fn assert_cstr(expected: &str, actual: *mut c_char) {
    unsafe {
        let msg = CStr::from_ptr(actual);
        assert_eq!(expected, msg.to_str().unwrap());
    }
}