mod ffi_util;

pub use ffi_util::*;
pub use std::ffi::CString;

#[cfg(feature = "derive")]
pub use foreignc_derive::*;