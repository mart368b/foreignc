use std::cell::RefCell;
use std::error::Error;
use crate::ArgResult;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use crate::*;

#[derive(Debug)]
#[repr(C)]
pub struct COption {
    inner_value: *mut c_void
}


thread_local! {
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

pub unsafe trait FromFFi<PtrIn> 
where
    Self: Sized
{
    fn from_ffi(v: PtrIn) -> ArgResult<Self>;
}

macro_rules! impl_direct {
    ($($T:ty),+) => {$(
        unsafe impl IntoFFi<$T> for $T {
            fn into_ffi(v: Self) -> $T { v }
        }

        unsafe impl IntoFFi<*mut c_void> for $T {
            fn into_ffi(v: Self) -> *mut c_void { Box::leak(Box::new( v )) as *mut $T as *mut c_void }
        }

        unsafe impl FromFFi<$T> for $T {
            fn from_ffi(v: $T) -> ArgResult<$T> { Ok(v) }
        }

        unsafe impl FromFFi<*mut c_void> for $T {
            fn from_ffi(v: *mut c_void) -> ArgResult<Self> { 
                Ok(unsafe {
                    *(v as *mut $T)
                        .as_ref()
                        .ok_or_else(|| ArgumentError::from("Recieved null pointer to primitive type"))?
                })
            }
        }
    )+}
}

impl_direct![
    bool,
    (),
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64
];

unsafe impl<T> IntoFFi<*mut T> for *mut T {
    fn into_ffi(v: Self) -> *mut T {
        v
    }
}

unsafe impl<T> IntoFFi<*const T> for *const T {
    fn into_ffi(v: Self) -> *const T {
        v
    }
}

unsafe impl IntoFFi<*mut c_void> for &str {
    fn into_ffi(v: Self) -> *mut c_void {
        IntoFFi::into_ffi(v.to_owned())
    }
}

unsafe impl<'a> FromFFi<*mut c_void> for &'a str {
    fn from_ffi(v: *mut c_void) -> ArgResult<&'a str> {
        Ok(unsafe {
            CStr::from_ptr(v as *mut c_char) 
        }.to_str()?)
    }
}

unsafe impl IntoFFi<*mut c_void> for String {
    fn into_ffi(v: Self) -> *mut c_void {
        CString::new(v).unwrap().into_raw() as *mut c_void
    }
}

unsafe impl FromFFi<*mut c_void> for String {
    fn from_ffi(v: *mut c_void) -> ArgResult<String> {
        let s: &str = FromFFi::from_ffi(v)?;
        Ok(s.to_owned())
    }
}

unsafe impl<T> FromFFi<*mut COption> for Option<T>
where
    T: FromFFi<*mut c_void> + std::fmt::Debug,
{
    fn from_ffi(v: *mut COption) -> ArgResult<Self> {
        if let Some(copt) = unsafe{v.as_ref()} {
            if !copt.inner_value.is_null() {
                Ok(Some(T::from_ffi(copt.inner_value)?))
            }else {
                Ok(None)
            }
        }else {
            Ok(None)
        }
    }
}

unsafe impl<T> IntoFFi<*mut c_void> for Option<T> 
where
    T: IntoFFi<*mut c_void>,
{
    fn into_ffi(v: Self) -> *mut c_void {
        Box::leak(Box::new(COption {
            inner_value: if let Some(v) = v {
                T::into_ffi(v)
            } else {
                std::ptr::null_mut()
            }
        })) as *mut COption as *mut c_void
    }
}

unsafe impl<T, E> IntoFFi<*mut c_void> for Result<T, E>
where
    E: Error + 'static
{
    fn into_ffi(v: Self) -> *mut c_void {
        std::ptr::null_mut()
    /*
        match v {
            Ok(v) => &mut T::into_ffi(v) as *mut Out,
            Err(e) => {
                update_last_error(e);
                std::ptr::null_mut()
            }
        }
        */
    }
}
