use crate::{MetaType, RustFreeFunction, RustFunction, RustStructure};
use std::fs::{create_dir, read_dir, read_to_string, remove_file, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn clean_up<P: AsRef<Path>>(p: P) {
    INIT.call_once(|| {
        for f in read_dir(p).unwrap() {
            remove_file(f.unwrap().path()).unwrap();
        }
    });
}

pub fn create_dir_path() -> PathBuf {
    let mut dir = Path::new(env!("OUT_DIR")).to_path_buf();
    dir.push("derived_data");
    if !dir.exists() {
        create_dir(&dir).unwrap();
    }
    dir.push(std::env::var("CARGO_PKG_NAME").unwrap());
    if !dir.exists() {
        create_dir(&dir).unwrap();
    }
    dir
}

pub fn create_file_path(name: String) -> PathBuf {
    let mut dir = create_dir_path();
    clean_up(&dir);
    dir.push(name);
    dir
}

#[derive(Default, Debug)]
pub struct ParsedFiles {
    pub structs: Vec<RustStructure>,
    pub functions: Vec<RustFunction>,
    pub free_functions: Vec<RustFreeFunction>,
}

fn open_file<P: AsRef<Path>>(p: P) -> File {
    OpenOptions::new().create(true).write(true).open(p).unwrap()
}

impl ParsedFiles {
    pub fn new(dir: &str) -> ParsedFiles {
        let mut pf = ParsedFiles::default();
        let pp = Path::new(&dir);
        if !pp.exists() {
            panic!(format!("{} does not exist", dir))
        }

        for p in read_dir(&pp).unwrap() {
            let p = p.unwrap().path();
            let s = read_to_string(p).unwrap();
            let v: MetaType = serde_json::from_str(&s).unwrap();
            match v {
                MetaType::FreeFunc(ff) => pf.free_functions.push(ff),
                MetaType::Func(f) => pf.functions.push(f),
                MetaType::Struct(s) => pf.structs.push(s),
            };
        }

        pf
    }

    #[cfg(feature = "derived_input")]
    pub fn add_struct(s: RustStructure) {
        let path = create_file_path(s.self_ty.to_owned());
        if !path.exists() {
            let w = open_file(path);
            let ss = &MetaType::Struct(s);
            serde_json::to_writer(w, &ss).unwrap();
        }
    }

    #[cfg(feature = "derived_input")]
    pub fn add_func(s: RustFunction) {
        let path = create_file_path(s.extern_name.to_owned());
        if !path.exists() {
            let w = open_file(path);
            let ss = &MetaType::Func(s);
            serde_json::to_writer(w, &ss).unwrap();
        }
    }

    #[cfg(feature = "derived_input")]
    pub fn add_free_func(s: RustFreeFunction) {
        let path = create_file_path(s.func.name.to_owned());
        if !path.exists() {
            let w = open_file(path);
            let ss = &MetaType::FreeFunc(s);
            serde_json::to_writer(w, &ss).unwrap();
        }
    }
}
