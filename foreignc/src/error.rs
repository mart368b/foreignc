use crate::{IntoFFi, COption};
use std::fmt::Display;
use std::os::raw::c_void;

pub type ArgResult<T> = Result<T, ArgumentError>;

pub struct ArgumentError {
    content: String
}

impl<T> From<T> for ArgumentError 
where
    T: Display
{
    fn from(v: T) -> ArgumentError {
        ArgumentError {
            content: format!("{}", v)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CArgResult {
    inner_value: *mut c_void,
    error: *mut c_void
}

impl<T> From<ArgResult<T>> for CArgResult
where
    T: IntoFFi<*mut c_void>
{
    fn from(v: ArgResult<T>) -> Self {
        match v {
            Ok(v) => CArgResult {
                inner_value: IntoFFi::into_ffi(Some(v)),
                error: IntoFFi::into_ffi(None::<Option<String>>),
            },
            Err(e) => 
            CArgResult {
                inner_value: IntoFFi::into_ffi(None::<Option<T>>),
                error: IntoFFi::into_ffi(Some(e.content)),
            },
        }
    }
}