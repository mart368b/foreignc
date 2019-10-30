use lazy_static::lazy_static;
use std::sync::Mutex;
use std::iter::FromIterator;
use crate::base::{ImplementBlock, RustFunction};

lazy_static! {
    static ref STRUCTS: Mutex<Vec<ImplementBlock>> = Mutex::new(Vec::new());
    static ref STRUCT_LOCK: Mutex<bool> = Mutex::new(false);
    static ref FUNCTIONS: Mutex<Vec<RustFunction>> = Mutex::new(Vec::new());
    static ref FUNC_LOCK: Mutex<bool> = Mutex::new(false);
}

pub fn add_impl(implm: ImplementBlock) {
    if *STRUCT_LOCK.lock().unwrap() {
        panic!("Structs have already been taken");
    }
    STRUCTS.lock().unwrap().push(implm);
}

pub fn add_func(func: RustFunction) {
    if *FUNC_LOCK.lock().unwrap() {
        panic!("Functions have already been taken");
    }
    FUNCTIONS.lock().unwrap().push(func);
}

pub fn take_impls() -> Vec<ImplementBlock> {
    let mut lock = STRUCT_LOCK.lock().unwrap();
    if *lock {
        panic!("Struct have already been taken");
    }
    *lock = true;
    let mut v = STRUCTS.lock().unwrap();
    let implm = Vec::from_iter(v.iter().map(|i| i.clone()));
    v.clear();
    implm
}

pub fn take_funcs() -> Vec<RustFunction> {
    let mut lock = FUNC_LOCK.lock().unwrap();
    if *lock {
        panic!("Functions have already been taken");
    }
    *lock = true;
    let mut v = FUNCTIONS.lock().unwrap();
    let funcs = Vec::from_iter(v.iter().map(|i| i.clone()));
    v.clear();
    funcs
}