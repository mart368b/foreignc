extern crate libc;
mod ffi_util;
mod error;

pub use error::*;
pub use ffi_util::*;
pub use std::ffi::CString;
pub use foreignc_derive::{
    generate_free_string, with_abi, Boxed, Json,
};
pub use libc::c_void;

pub unsafe fn free_libc(v: *mut c_void) {
    libc::free(v);
}

#[cfg(feature = "template")]
pub use ffi_template::*;


#[cfg(feature = "template")]
use ffi_template::derived_input::{get_dir_path};
#[cfg(feature = "template")]
use ffi_template::RustContext;

#[cfg(feature = "template")]
pub fn get_package_dir() -> TResult<RustContext> {
    let pkg_name = std::env::var("CARGO_PKG_NAME")?;
    let dir = get_dir_path(pkg_name)?;
    let dir_str = dir.to_str().unwrap();
    RustContext::from_path_directory(dir_str)
}

#[cfg(feature = "template")]
pub fn get_parsed_dir(name: String) -> TResult<RustContext> {
    let dir = get_dir_path(name)?;
    let dir_str = dir.to_str().unwrap();
    RustContext::from_path_directory(dir_str)
}
