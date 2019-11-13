use std::cell::RefCell;
use std::error::Error;
use crate::FFiResult;
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::*;


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
    fn into_ffi(v: Self) -> FFiResult<PtrOut>;
}

pub unsafe trait FromFFi<PtrIn> 
where
    Self: Sized
{
    fn from_ffi(v: PtrIn) -> FFiResult<Self>;
}

macro_rules! impl_direct {
    ($($T:ty),+) => {$(
        unsafe impl IntoFFi<$T> for $T {
            fn into_ffi(v: Self) -> FFiResult<$T> { Ok(v) }
        }

        unsafe impl IntoFFi<*mut $T> for $T {
            fn into_ffi(v: Self) -> FFiResult<*mut $T> { Ok(Box::leak(Box::new(v))) }
        }

        unsafe impl FromFFi<$T> for $T {
            fn from_ffi(v: $T) -> FFiResult<$T> { Ok(v) }
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
    fn into_ffi(v: Self) -> FFiResult<*mut T> {
        Ok(v)
    }
}

unsafe impl<T> IntoFFi<*const T> for *const T {
    fn into_ffi(v: Self) -> FFiResult<*const T> {
        Ok(v)
    }
}

unsafe impl IntoFFi<*mut c_char> for &str {
    fn into_ffi(v: Self) -> FFiResult<*mut c_char> {
        IntoFFi::into_ffi(v.to_owned())
    }
}

unsafe impl<'a> FromFFi<*mut c_char> for &'a str {
    fn from_ffi(v: *mut c_char) -> FFiResult<&'a str> {
        Ok(unsafe {
            CStr::from_ptr(v)
        }.to_str()?)
    }
}

unsafe impl IntoFFi<*mut c_char> for String {
    fn into_ffi(v: Self) -> FFiResult<*mut c_char> {
        Ok(CString::new(v)?.into_raw())
    }
}

unsafe impl FromFFi<*mut c_char> for String {
    fn from_ffi(v: *mut c_char) -> FFiResult<String> {
        let s: &str = FromFFi::from_ffi(v)?;
        Ok(s.to_owned())
    }
}

unsafe impl<T, U> FromFFi<*mut U> for Option<T>
where
    T: FromFFi<U> + std::fmt::Debug,
{
    fn from_ffi(v: *mut U) -> FFiResult<Self> {
        unsafe{
            match v.as_mut() {
                Some(ptr) => Ok(Some(T::from_ffi(std::ptr::read(ptr))?)),
                None => Ok(None)
            }
        }
    }
}

unsafe impl<T, U> IntoFFi<*mut U> for Option<T> 
where
    T: IntoFFi<U>,
{
    fn into_ffi(v: Self) -> FFiResult<*mut U> {
        Ok(if let Some(v) = v {
            Box::leak(Box::new(T::into_ffi(v)?))
        } else {
            std::ptr::null_mut()
        })
    }
}
