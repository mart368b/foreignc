/// # ffi-template
/// Allow for generation of python apis using a simple representation of rust code

mod rust_types;
mod py_types;
mod py_gen;

mod error;
pub mod derived_input;

pub use error::*;
pub use rust_types::*;
pub use py_gen::*;
