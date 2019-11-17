mod ffi_util;
mod error;

pub use error::*;
pub use ffi_util::*;
pub use std::ffi::CString;
pub use ffi_template::*;

pub use foreignc_derive::{
    generate_free_string, generate_last_error, with_abi, Boxed, Json,
};

#[allow(unused_imports)]
use ffi_template::derived_input::{get_dir_path};
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
