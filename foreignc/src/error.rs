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


impl<T> Into<*mut CResult<T, c_char>> for FFiResultWrap<T> {
    fn into(self) -> *mut CResult<T, c_char> {
        Box::leak(Box::new(match self.0 {
            Ok(v) => {
                CResult {
                    ok: Box::leak(Box::new(v)),
                    err: std::ptr::null_mut()
                }
            },
            Err(e) => {
                CResult {
                    ok: std::ptr::null_mut(),
                    err: IntoFFi::into_ffi(e.content).unwrap()
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
        FFiError {
            content: format!("{}", v)
        }
    }
}
