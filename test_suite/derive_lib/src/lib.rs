mod base_test;
mod serde_test;
mod impl_test;
mod simple_error;

pub use simple_error::SimpleError;
pub use foreignc::*;

generate_free_string!();
generate_last_error!();
