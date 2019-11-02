use lazy_static::lazy_static;
use std::sync::Mutex;
use std::iter::FromIterator;
use crate::{RustFunction, RustStructure, RustFreeFunction};

lazy_static! {
    static ref STRUCTS: Mutex<Vec<RustStructure>> = Mutex::new(Vec::new());
    static ref STRUCT_LOCK: Mutex<bool> = Mutex::new(false);
    static ref FUNCTIONS: Mutex<Vec<RustFunction>> = Mutex::new(Vec::new());
    static ref FUNC_LOCK: Mutex<bool> = Mutex::new(false);
    static ref FREE_FUNCTIONS: Mutex<Vec<RustFreeFunction>> = Mutex::new(Vec::new());
    static ref FREE_FUNC_LOCK: Mutex<bool> = Mutex::new(false);
}

pub fn add_struct(implm: RustStructure) {
    if *STRUCT_LOCK.lock().unwrap() {
        panic!("Structs have already been taken");
    }
    STRUCTS.lock().unwrap().push(implm);
}

pub fn add_free_func(func: RustFreeFunction) {
    if *FREE_FUNC_LOCK.lock().unwrap() {
        panic!("Functions have already been taken");
    }
    FREE_FUNCTIONS.lock().unwrap().push(func);
}

pub fn add_func(func: RustFunction) {
    if *FUNC_LOCK.lock().unwrap() {
        panic!("Functions have already been taken");
    }
    FUNCTIONS.lock().unwrap().push(func);
}

pub fn take_free_funcs() -> Vec<RustFreeFunction> {
    let mut lock = FREE_FUNC_LOCK.lock().unwrap();
    if *lock {
        panic!("Struct have already been taken");
    }
    *lock = true;
    let mut v = FREE_FUNCTIONS.lock().unwrap();
    let funcs = Vec::from_iter(v.iter().map(|i| i.clone()));
    v.clear();
    funcs
}

pub fn take_structs() -> Vec<RustStructure> {
    let mut lock = STRUCT_LOCK.lock().unwrap();
    if *lock {
        panic!("Struct have already been taken");
    }
    *lock = true;
    let mut v = STRUCTS.lock().unwrap();
    let structs = Vec::from_iter(v.iter().map(|i| i.clone()));
    v.clear();
    structs
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