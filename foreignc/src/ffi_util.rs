extern crate libc;
use crate::FFiResult;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::marker::PhantomData;
use crate::*;
use std::mem;

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
            let v = IntoFFi::into_ffi(v)?;
            unsafe {
                let obj_size = mem::size_of_val(&v);
                println!("option size: {}", obj_size);
                let ptr: *mut U = libc::malloc(obj_size) as *mut U;
                *ptr = v;
                ptr
            }
        } else {
            std::ptr::null_mut()
        })
    }
}

#[repr(C)]
pub struct CResult<T, E>{
    pub is_err: bool,
    pub value: *mut c_void,
    pub t: PhantomData<T>,
    pub e: PhantomData<E>,
}

unsafe impl<T, E, U, V> IntoFFi<*mut CResult<*mut U, *mut V>> for Result<T, E> 
where
    T: IntoFFi<U>,
    E: IntoFFi<V>,
{
    fn into_ffi(v: Self) -> FFiResult<*mut CResult<*mut U, *mut V>> {
        unsafe {
            let v = match v {
                Ok(v) => {
                    let v = IntoFFi::into_ffi(v)?;
                    let obj_size = mem::size_of_val(&v);
                    println!("result size: {}", obj_size);
                    let ptr: *mut U = libc::malloc(obj_size) as *mut U;
                    *ptr = v;
                    CResult {
                        is_err: false,
                        value: ptr as *mut c_void,
                        t: PhantomData,
                        e: PhantomData,
                    }
                },
                Err(v) => {
                    let v = IntoFFi::into_ffi(v)?;
                    let obj_size = mem::size_of_val(&v);
                    let ptr: *mut V = libc::malloc(obj_size) as *mut V;
                    *ptr = v;
    
                    CResult {
                        is_err: true,
                        value: ptr as *mut c_void,
                        t: PhantomData,
                        e: PhantomData,
                    }
                }
            };
            
            let obj_size = mem::size_of_val(&v);
            let ptr = libc::malloc(obj_size) as *mut CResult<*mut U, *mut V>;
            *ptr = v;

            Ok(ptr)
        }
    }
}