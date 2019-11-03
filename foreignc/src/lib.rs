mod ffi_util;

pub use ffi_util::*;
pub use std::ffi::CString;

pub use ffi_template::*;

pub use foreignc_derive::{
    generate_free_string, generate_last_error, inspect, wrap_extern, Boxed, Json,
};

#[allow(unused_imports)]
use ffi_template::derived_input::{get_dir_path, ParsedPathFiles};

#[cfg(feature = "template")]
pub fn get_package_dir() -> TResult<ParsedPathFiles> {
    let pkg_name = std::env::var("CARGO_PKG_NAME")?;
    let dir = get_dir_path(pkg_name)?;
    let dir_str = dir.to_str().unwrap();
    ParsedPathFiles::from_path_directory(dir_str)
}

#[cfg(feature = "template")]
pub fn get_parsed_dir(name: String) -> TResult<ParsedPathFiles> {
    let dir = get_dir_path(name)?;
    let dir_str = dir.to_str().unwrap();
    ParsedPathFiles::from_path_directory(dir_str)
}
