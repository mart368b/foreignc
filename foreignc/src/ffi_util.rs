
use std::ffi::CStr;
use std::os::raw::c_char;
use std::cell::RefCell;
use std::error::Error;

use crate::*;


thread_local!{
    static LAST_ERROR: RefCell<Option<Box<dyn Error>>> = RefCell::new(None);
}

pub fn update_last_error<E: Error + 'static>(err: E) {

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // cause until we reach the root error.
        let mut cause = err.source();
        while let Some(parent_err) = cause {
            cause = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<dyn Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

pub unsafe trait IntoFFi<PtrOut> {
    fn into_ffi(v: Self) -> PtrOut;
}

pub unsafe trait FromFFi<PtrIn> {
    fn from_ffi(v: PtrIn) -> Self;
}

pub unsafe trait FFiDefault {
    fn default() -> Self;
}

macro_rules! impl_direct {
    ($($T:ty),+) => {$(
        unsafe impl IntoFFi<$T> for $T {
            fn into_ffi(v: Self) -> $T { v }
        }

        unsafe impl FromFFi<$T> for $T {
            fn from_ffi(v: $T) -> Self { v }
        }
    )+}
}

impl_direct![
    bool, (), 
    i8, u8, i16, 
    u16, i32, u32, i64, 
    u64, f32, f64, *mut i8, 
    *const i8, *mut u8, *const u8
];

macro_rules! impl_default {
    ($($T:ident),+) => {$(
        unsafe impl FFiDefault for $T {
            fn default() -> Self { std::$T::MAX }
        }
    )+}
}

impl_default![
    i8, u8, i16, 
    u16, i32, u32, i64, 
    u64, f32, f64
];

unsafe impl<T> FFiDefault for *mut T {
    fn default() -> Self { std::ptr::null_mut() }
}

unsafe impl<T> FFiDefault for *const T {
    fn default() -> Self { std::ptr::null() }
}

unsafe impl IntoFFi<*mut c_char> for &str {
    fn into_ffi(v: Self) -> *mut c_char { 
        IntoFFi::into_ffi(v.to_owned())
    }
}

unsafe impl<'a> FromFFi<*const c_char> for &'a str {
    fn from_ffi(v: *const c_char) -> &'a str {
        unsafe { CStr::from_ptr(v) }.to_str().unwrap()
    }
}

unsafe impl IntoFFi<*mut c_char> for String {
    fn into_ffi(v: Self) -> *mut c_char { 
        CString::new(v).unwrap().into_raw()
    }
}

unsafe impl<'a> FromFFi<*const c_char> for String {
    fn from_ffi(v: *const c_char) -> String {
        let s: &str = FromFFi::from_ffi(v);
        s.to_owned()
    }
}

unsafe impl<T, In> FromFFi<In> for Option<T> 
where
    In: FFiDefault + Eq,
    T: FromFFi<In>
{
    fn from_ffi(v: In) -> Self {
        if v == In::default()  {
            Some(T::from_ffi(v))
        }else {
            None
        }
    }
}

unsafe impl<T, Out> IntoFFi<Out> for Option<T> 
where
    Out: FFiDefault,
    T: IntoFFi<Out>
{
    fn into_ffi(v: Self) -> Out {
        if let Some(v) = v {
            T::into_ffi(v)
        }else {
            Out::default()
        }
    }
}

unsafe impl<T, Out, E> IntoFFi<Out> for Result<T, E> 
where
    Out: FFiDefault,
    E: Error + 'static,
    T: IntoFFi<Out> + Default
{
    fn into_ffi(v: Self) -> Out {
        match v {
            Ok(o) => T::into_ffi(o),
            Err(e) => {
                update_last_error(e);
                Out::default()
            }
        }
    }
}