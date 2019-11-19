pub use foreignc::*;
use std::panic;
use serde::{Deserialize, Serialize};

generate_free_string!();

#[derive(Boxed, Serialize, Deserialize, Debug)]
pub struct BoxedStruct{
    name: String,
    value: String
}

#[derive(Json, Serialize, Deserialize, Debug)]
pub struct JsonStruct{
    name: String,
    value: String
}
/*

#[with_abi]
impl JsonStruct {
    pub fn new() -> JsonStruct {
        JsonStruct {
            name: "Hello".to_owned(),
            value: "World!".to_owned()
        }
    }

    pub fn debug(self) {
        println!("debug: {:?}", self);
    }
}

#[with_abi]
impl BoxedStruct {
    pub fn new() -> BoxedStruct {
        BoxedStruct {
            name: "Boxed".to_owned(),
            value: "World!".to_owned()
        }
    }

    pub fn debug(&self) {
        println!("debug: {:?}", self);
    }
}
*/

pub fn does_panic() -> &'static str {
    panic!("a");
}
#[no_mangle]
pub  extern "C" fn does_panic_ffi(
) -> *mut foreignc::CResult<*mut std::os::raw::c_char, std::os::raw::c_char> {
    unsafe {
        let v = panic::catch_unwind(|| -> foreignc::FFiResult<_> {
            Ok(foreignc::IntoFFi::into_ffi(does_panic())?)
        })
        .unwrap_or_else(|e| {
            Err(foreignc::FFiError {
                content: "Panic".to_owned(),
            })
        });
        foreignc::FFiResultWrap::from(v).into()
    }
}

/*
#[with_abi]
pub fn does_panic() -> &'static str {
    ""
}
#[with_abi]
pub fn get_string() -> &'static str {
    "Hello World!"
}

#[with_abi]
pub fn parse_string(s: String) {
    println!("a + {}", s);
}

#[with_abi]
pub fn get_number() -> u32 {
    123456
}

#[with_abi]
pub fn get_none() -> Option<u32> {
    None
}

#[with_abi]
pub fn get_some() -> Option<String> {
   Some("Some(123456)".to_owned())
}

#[with_abi]
pub fn set_some(v: Option<String>) {
    println!("---{:?}", v);
}
*/
