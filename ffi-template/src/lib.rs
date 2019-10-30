mod base;
mod python;

#[cfg(feature = "derived_input")]
pub mod derived_input;

use std::fmt::Debug;

pub use base::*;
pub use python::*;

#[derive(Default, Debug, Clone)]
pub struct Argument<T> 
where
    T: Default + Debug
{
    pub name: String,
    pub ty: T,
}
