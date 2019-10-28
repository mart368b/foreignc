use std::os::raw::c_char;
use std::ffi::CString;

#[link(name = "test_lib.dll")]
extern "C" {
    pub fn free_string(ptr: *mut c_char);
    pub fn last_error() -> *mut c_char;
    pub fn hello_world() -> *mut c_char;
    pub fn add(a: i32, b: i32) -> i32;
    pub fn throw_err(is_err: bool) -> *mut c_char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        unsafe {
            let s = hello_world();
            let msg = CString::from_raw(s);
            assert_eq!("Hello World!", msg.to_str().unwrap());
        }
    }

    #[test]
    fn add_test() {
        unsafe {
            assert_eq!(2, add(1, 1));
        }
    }

    #[test]
    fn result_test() {
        unsafe {
            let ok = throw_err(false);
            assert!(!ok.is_null());
            let ok_msg = CString::from_raw(ok);
            assert_eq!("Ok", ok_msg.to_str().unwrap());


            let err = throw_err(true);
            assert!(err.is_null());
            let lerr = last_error();
            let err_msg = CString::from_raw(lerr);
            assert_eq!("Err", err_msg.to_str().unwrap());
        }
    }
}