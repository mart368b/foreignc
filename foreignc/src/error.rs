use std::fmt::Display;
use std::os::raw::c_void;

pub type FFiResult<T> = Result<T, FFiError>;

#[derive(Debug)]
pub struct FFiError {
    content: String
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

#[derive(Debug)]
#[repr(C)]
pub struct CFFiResult {
    inner_value: *mut c_void,
    error: *mut c_void
}
/*
impl<T> From<FFiResult<T>> for CFFiResult
where
    T: IntoFFi<*mut c_void>
{
    fn from(v: FFiResult<T>) -> Self {
        match v {
            Ok(v) => CFFiResult {
                inner_value: IntoFFi::into_ffi(Some(v)),
                error: IntoFFi::into_ffi(None::<Option<String>>),
            },
            Err(e) => 
            CFFiResult {
                inner_value: IntoFFi::into_ffi(None::<Option<T>>),
                error: IntoFFi::into_ffi(Some(e.content)),
            },
        }
    }
}
*/