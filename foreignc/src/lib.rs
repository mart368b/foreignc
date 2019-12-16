//! # Foreignc
//! Foreignc is a framework for auto generating a safe ffi abi for rust methods.
//! The main advantage if foreignc is that allows for easy deplayment and maintenance of safe ffi abi.
//! The crate is made up of two parts.
//!  - Macros for auto generating ffi abis and cleanup methods
//!  - Functions for auto generate the recieving end of the ffi abi
//!     - Currently only python is supported
//! 
//! # Quick start
//! ## Example
//! 
//! ```rust
//! # pub use foreignc::*;
//! 
//! // Create free methods
//! generate_free_methods!();
//! 
//! // Evaluates to hello_world() -> CResult<()>
//! #[with_abi]
//! pub fn hello_world() {
//!     println!("Hello World");
//! }
//! 
//! 
//! #[derive(Boxed, Debug)]
//! pub struct BoxedCounter{
//!     value: u32
//! }
//! 
//! #[with_abi]
//! impl BoxedCounter {
//!     // Evaluates to new_boxed_counter() -> CResult<*mut BoxedCounter>
//!     pub fn new() -> BoxedCounter{
//!         BoxedCounter {
//!             value: 0
//!         }
//!     }
//! 
//!     // Evaluates to inc_boxed_counter(*mut BoxedCounter) -> CResult<()>
//!     pub fn inc(&mut self) {
//!         self.value += 1;
//!     }
//! 
//!     // Evaluates to inc_boxed_counter(*mut BoxedCounter) -> CResult<()>
//!     pub fn display(&self) {
//!         println!("{:?}", self);
//!     }
//! }
//! ```
//! 
//! The above example will generate a ffi abi, that when called returns a CResult indicating wether the call ended succesfully or not.
//! When arguments are parsed parsed to the function the FromFFi trait is used to convert from a unsafe value into a safe one
//! When values are returned IntoFFi is used, to convert the type into a ffi safe value.
//! 
//! ## Templating
//! using the feature 'template' it is possible to auto generate the recieving side of the ffi.
//! 
//! ### Example
//! add `build = "build.rs"` to the package section of cargo.toml
//! 
//! ***build.rs***
//! ```rust
//! use foreignc::get_package_dir;
//! 
//! fn main() {
//!     // Get all the abis that have been created
//!     let resource = get_package_dir().unwrap();
//!     // Create the python api
//!     resource.generate_python_api("api.py", None).unwrap();
//! }
//! ```
//! 
//! after running ```cargo build``` a api.py file is created that looks like this:
//! 
//! ```python
//! from __future__ import annotations
//! from foreignc import *
//! 
//! class BoxedCounter(Box):
//!     @staticmethod
//!     def __free_func__() -> str:
//!         return 'free_boxed_counter'
//! 
//!     @create_abi('new_boxed_counter', restype='BoxedCounter')
//!     def new(lib: BaseLib) -> BoxedCounter:
//!         return lib.__lib__.new_boxed_counter().consume()
//! 
//!     @create_abi('inc_boxed_counter', argtypes=('BoxedCounter',))
//!     def inc(self) :
//!         return self.__lib__.inc_boxed_counter(self).consume()
//! 
//!     @create_abi('display_boxed_counter', argtypes=('BoxedCounter',))
//!     def display(self) :
//!         return self.__lib__.display_boxed_counter(self).consume()
//! 
//! submit_type('BoxedCounter', BoxedCounter)
//! 
//! class MyCrateNameLib(BaseLib):
//!     def __init__(self, src: str):
//!         super().__init__(src)
//! 
//!     @create_abi('hello_world_ffi')
//!     def hello_world(self) :
//!         return self.__lib__.hello_world_ffi().consume()
//! ```
//! 
//! Then to run using python:
//!  1. install foreignc `pip install foreignc`
//!  2. Use the api
//! ```python
//! from api import MyCrateNameLib, BoxedCounter
//! 
//! library_path = "C:/Somepath/file.dll"
//! lib = MyCrateNameLib(library_path)
//! 
//! # Call hello world
//! lib.hello_world()
//! 
//! counter = BoxedCounter(lib)
//! 
//! # prints 0
//! counter.display()
//! counter.inc()
//! 
//! # prints 1
//! counter.display()
//! 
//! # BoxedCounter droped when the garbage collector run
//! ```
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
//! 
//! To parse custom struct accross the ffi barrier use Boxed or Json as such
//! ```rust
//! pub use foreignc::*;
//! generate_free_methods!();
//! pub use serde::{Serialize, Deserialize};
//!    
//! #[derive(Boxed)]
//! pub struct BoxedCounter{
//!     value: u32
//! }
//! 
//! // Wil auto generate free method free_boxed_counter
//! 
//! #[with_abi]
//! impl BoxedCounter {
//!     pub fn inc(&mut self) {
//!         self.value += 1;
//!     }
//! }
//! 
//! #[derive(Json, Serialize, Deserialize)]
//! pub struct JsonCounter{
//!     value: u32
//! }
//! 
//! impl JsonCounter {
//!     pub fn inc(mut self) -> JsonCounter {
//!         self.value += 1;
//!         self
//!     }
//! }
//! ```
//! 
//! Boxed structs are wrapped in a Box that stores the struct on the heap.
//! Json converts the struct to a string using serde everytime it needs to parse the ffi barrier
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

extern crate libc;
mod ffi_util;
mod error;

pub use error::*;
pub use ffi_util::*;
pub use std::ffi::CString;
pub use foreignc_derive::{
    generate_free_methods, with_abi, Boxed, Json,
};
pub use libc::c_void;
pub use libc::free as free_libc;

#[cfg(feature = "template")]
pub use ffi_template::*;

#[cfg(feature = "template")]
use ffi_template::derived_input::{get_dir_path};
#[cfg(feature = "template")]
use ffi_template::RustContext;

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
