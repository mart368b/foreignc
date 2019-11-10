use std::cell::RefCell;
use std::error::Error;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::marker::PhantomData;

use crate::*;

#[repr(C)]
pub struct COption {
    content: *mut c_void
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

pub unsafe trait IntoFFi {
    type PtrOut;
    fn into_ffi(v: Self) -> Self::PtrOut;
}

pub unsafe trait FromFFi {
    type PtrIn;
    fn from_ffi(v: Self::PtrIn) -> Self;
}

macro_rules! impl_direct {
    ($($T:ty),+) => {$(
        unsafe impl IntoFFi for $T {
            type PtrOut = $T;
            fn into_ffi(v: Self) -> Self::PtrOut { v }
        }

        unsafe impl FromFFi for $T {
            type PtrIn = $T;
            fn from_ffi(v: Self::PtrIn) -> Self { v }
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

unsafe impl<T> IntoFFi for *mut T {
    type PtrOut = *mut T;
    fn into_ffi(v: Self) -> Self::PtrOut {
        v
    }
}

unsafe impl<T> IntoFFi for *const T {
    type PtrOut = *const T;
    fn into_ffi(v: Self) -> Self::PtrOut {
        v
    }
}

unsafe impl IntoFFi for &str {
    type PtrOut = *mut c_char;
    fn into_ffi(v: Self) -> Self::PtrOut {
        IntoFFi::into_ffi(v.to_owned())
    }
}

unsafe impl<'a> FromFFi for &'a str {
    type PtrIn = *mut c_char;
    fn from_ffi(v: Self::PtrIn) -> &'a str {
        unsafe { CStr::from_ptr(v) }.to_str().expect("Failed to parse string as utf-8")
    }
}

unsafe impl IntoFFi for String {
    type PtrOut = *mut c_char;
    fn into_ffi(v: Self) -> Self::PtrOut {
        CString::new(v).unwrap().into_raw()
    }
}

unsafe impl FromFFi for String {
    type PtrIn = *mut c_char;
    fn from_ffi(v: Self::PtrIn) -> String {
        let s: &str = FromFFi::from_ffi(v);
        s.to_owned()
    }
}

unsafe impl<T> FromFFi for Option<T>
where
    T: FromFFi,
    <T as FromFFi>::PtrIn: From<*mut c_void>
{
    type PtrIn = *mut COption;
    fn from_ffi(v: Self::PtrIn) -> Self {
        unsafe{
            let ptr = v.as_mut().unwrap();
            //ptr.content.as_mut().map(|v| T::from_ffi(<T as FromFFi>::PtrIn::from(v)))
        };
        None
    }
}

unsafe impl<T> IntoFFi for Option<T> 
where
    T: IntoFFi,
{
    type PtrOut = *mut COption;
    fn into_ffi(v: Self) -> Self::PtrOut {
        Box::leak(Box::new(COption {
            content: if let Some(v) = v {
                (Box::leak(Box::new(T::into_ffi(v))) as *mut <T as ffi_util::IntoFFi>::PtrOut) as *mut c_void
            } else {
                std::ptr::null_mut()
            }
        })) as *mut COption
    }
}

unsafe impl<T, E> IntoFFi for Result<T, E>
where
    E: Error + 'static,
    T: IntoFFi,
{
    type PtrOut = *mut c_void;
    fn into_ffi(v: Self) -> Self::PtrOut {
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
