//! # Foreignc
//! Foreignc is a framework for auto generating a safe ffi abi for rust methods.
//! The main advantage if foreignc is that allows for easy deplayment and maintenance of safe ffi abi.
//! The crate is made up of two parts.
//!  - Macros for auto generating ffi abis and cleanup methods
//!  - Functions for auto generate the recieving end of the ffi abi
//!     - Currently only python is supported

//! # Templating
//! Using the feature 'template' it is possible to auto generate the recieving side of the ffi.
//! This will also add two new functions get_package_dir and get_parsed_dir. Both functions return a representation of the current ffi api
//!
//!
//! # Default types
//! ## Primititve types
//! The following primitive types are supported:
//!  - bool
//!  - ()
//!  - i8, i16, i32, i64
//!  - u8, u16, u32, u64
//!  - f32, f64
//!  - &str (will be converted to a CString)
//!
//! ## Other types
//! The following other types are soppurted:
//! - Result (will be converted to a CResult)
//! - Option (will be converted to a COption)
//! - String (will be converted to a CString)
//!
//! # Custom Structs
//! Custom types can be implemented either by using the IntoFFi, FromFFi trait or the Boxed, Json macro.
//!
//! # Safety
//! As a rule of thumb, all allocated memory needs too be unallocated by the creator.
//! This is also the basis for generate_free_methods that creates frunctions for freeing memory allocated by structures made by foreignc
//! The following functions are made
//!  - free_string(ptr: *mut CString)
//!  - free_coption(ptr: *mut COption)
//!  - free_cresult(ptr: *mut CResult)
//!
//! Boxed structs will auto generate a free method using the following convention free_{to_snake_case(struct name)}
//!
//! For more information see the [![git repository]](https://github.com/mart368b/foreignc)

extern crate libc;
mod error;
mod ffi_util;

pub use error::*;
pub use ffi_util::*;
pub use foreignc_derive::{generate_free_methods, with_abi, Boxed, Json};
pub use libc::c_void;
pub use libc::free as free_libc;
pub use std::ffi::CString;

#[cfg(feature = "template")]
pub use foreignc_template::*;

#[cfg(feature = "template")]
use foreignc_template::derived_input::get_dir_path;
#[cfg(feature = "template")]
use foreignc_template::RustContext;

#[cfg(feature = "template")]
/// Get the parsed abis from the current crate
pub fn get_package_dir() -> TResult<RustContext> {
    let pkg_name = std::env::var("CARGO_PKG_NAME")?;
    let dir = get_dir_path(pkg_name)?;
    let dir_str = dir.to_str().unwrap();
    RustContext::from_path_directory(dir_str)
}

#[cfg(feature = "template")]
/// Get the parsed abis from the provided crate
pub fn get_parsed_dir(name: String) -> TResult<RustContext> {
    let dir = get_dir_path(name)?;
    let dir_str = dir.to_str().unwrap();
    RustContext::from_path_directory(dir_str)
}
