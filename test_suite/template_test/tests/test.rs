use foreignc::*;
use std::sync::Mutex;
use foreignc::derived_input::{take_funcs, take_free_funcs, take_structs};
use lazy_static::lazy_static;
use std::sync::Once;

static INIT: Once = Once::new();

lazy_static! {
    static ref STRUCTS: Mutex<Vec<RustStructure>> = Mutex::new(Vec::new());
    static ref FUNCTIONS: Mutex<Vec<RustFunction>> = Mutex::new(Vec::new());
    static ref FREE_FUNCTIONS: Mutex<Vec<RustFreeFunction>> = Mutex::new(Vec::new());
}

fn setup() {
    INIT.call_once(|| {
        STRUCTS.lock().unwrap().append(&mut take_structs());
        FUNCTIONS.lock().unwrap().append(&mut take_funcs());
        FREE_FUNCTIONS.lock().unwrap().append(&mut take_free_funcs());
    });
}

fn get_resources() -> (Vec<RustStructure>, Vec<RustFunction>, Vec<RustFreeFunction>) {
    setup();
    (
        STRUCTS.lock().unwrap().clone(),
        FUNCTIONS.lock().unwrap().clone(),
        FREE_FUNCTIONS.lock().unwrap().clone()
    )
}

#[test]
#[should_panic]
fn second_take_structs() {
    setup();
    take_structs();
}

#[test]
#[should_panic]
fn second_take_funcs() {
    setup();
    take_funcs();
}

#[test]
#[should_panic]
fn second_take_free_funcs() {
    setup();
    take_free_funcs();
}