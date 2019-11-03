mod ffi_util;

pub use ffi_util::*;
pub use std::ffi::CString;

pub use ffi_template::*;

pub use foreignc_derive::{
    generate_free_string, generate_last_error, inspect, wrap_extern, Boxed, Json,
};

#[allow(unused_imports)]
use ffi_template::derived_input::{create_dir_path, ParsedFiles};

#[cfg(feature = "template")]
pub fn get_parsed_dir() -> ParsedFiles {
    ParsedFiles::new(create_dir_path().to_str().unwrap())
}
