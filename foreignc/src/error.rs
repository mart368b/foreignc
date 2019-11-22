use crate::{CResult, IntoFFi};
use std::fmt::Display;
use std::os::raw::c_char;

pub type FFiResult<T> = Result<T, FFiError>;
pub struct FFiResultWrap<T>(FFiResult<T>);

impl<T> From<FFiResult<T>> for FFiResultWrap<T> {
    fn from(v: FFiResult<T>) -> Self {
        Self(v)
    }
}


impl<T> Into<*mut CResult<T, *mut c_char>> for FFiResultWrap<T> {
    fn into(self) -> *mut CResult<T, *mut c_char> {
        Box::into_raw(Box::new(match self.0 {
            Ok(v) => {
                CResult {
                    is_err: false,
                    ok: Box::into_raw(Box::new(v)),
                    err: std::ptr::null_mut()
                }
            },
            Err(e) => {
                CResult {
                    is_err: true,
                    ok: std::ptr::null_mut(),
                    err: Box::into_raw(Box::new(String::into_ffi(e.content).unwrap()))
                }
            }
        }))
    }
}

#[derive(Debug)]
pub struct FFiError {
    pub content: String
}

impl<T> From<T> for FFiError 
where
    T: Display
{
    fn from(v: T) -> FFiError {
        println!("Creating error {}", v);
        FFiError {
            content: format!("{}", v)
        }
    }
}
