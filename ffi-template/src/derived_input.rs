use crate::*;
#[allow(unused_imports)]
use std::fs::{create_dir, read_dir, read_to_string, remove_file, File, OpenOptions};
#[allow(unused_imports)]
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

static INIT: Once = Once::new();

fn clean_up<P: AsRef<Path>>(p: P){
    INIT.call_once(|| {
        let path = p.as_ref();
        match read_dir(path) {
            Ok(dir) => {
                for f in dir {
                    match f {
                        Ok(f) => if let Err(e) = remove_file(f.path()) {
                            println!("Failed to remove file {:?} because {:?}", f.path(), e);
                        },
                        Err(e) => println!("Failed to read file because {:?}", e)
                    }
                }
            },
            Err(e) => println!("Failed to clear {:?} because {:?}", path, e)
        }
        
    });
}

pub fn get_dir_path(name: String) -> TResult<PathBuf> {
    let mut dir = Path::new(env!("OUT_DIR")).to_path_buf();
    dir.push("derived_data");
    if !dir.exists() {
        create_dir(&dir)?;
    }
    dir.push(name);
    if !dir.exists() {
        create_dir(&dir)?;
    }
    Ok(dir)
}


pub fn get_file_path(name: String, body: &str) -> TResult<PathBuf> {
    let mut s = DefaultHasher::new();
    body.hash(&mut s);
    let h = s.finish();
    let pkg = std::env::var("CARGO_PKG_NAME")?;
    let mut dir = get_dir_path(pkg)?;
    clean_up(&dir);
    dir.push(name + &h.to_string());
    Ok(dir)
}

fn open_file<P: AsRef<Path>>(p: P) -> TResult<File> {
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .open(p)?)
}

pub fn add_to_path<T>(s: T) -> TResult<()> 
where
    T: Into<MetaType>
{   
    let meta: MetaType = s.into();
    let filename = match meta {
        MetaType::FreeFunc(ref ff) => ff.func.extern_name.to_owned(),
        MetaType::Func(ref f) => f.extern_name.to_owned(),
        MetaType::Struct(ref s) => s.self_ty.to_owned(),
    };
    let body = serde_json::to_string(&meta)?;
    let path = get_file_path(filename, &body)?;
    if !path.exists() {
        let mut w = open_file(path)?;
        let bbody = body.as_bytes();
        w.write_all(bbody)?;
    }
    Ok(())
}

impl RustContext {

    pub fn from_path_directory(dir: &str) -> TResult<Self> {
        let mut pf = Self::new();
        let pp = Path::new(&dir);
        if !pp.exists() {
            panic!(format!("{} does not exist", dir))
        }

        for p in read_dir(&pp)? {
            let path = p?.path();
            let s = read_to_string(path)?;
            let v: MetaType = serde_json::from_str(&s)?;
            match v {
                MetaType::FreeFunc(ff) => pf.free_funcs.push(ff),
                MetaType::Func(f) => pf.funcs.push(f),
                MetaType::Struct(s) => pf.add_struct(s),
            };
        }

        Ok(pf)
    }

    pub fn add_struct(&mut self, mut s: RustStructure) {
        let cmp = |v: &RustStructure, o: &RustStructure| v.self_ty.cmp(&o.self_ty);
        match self.structs.binary_search_by(|prope| cmp(prope, &s)) {
            Ok(i) => {
                self.structs[i].methods.append(&mut s.methods);
                if let Some(destructor) = s.destructor {
                    self.structs[i].destructor = Some(destructor);
                }
            },
            Err(i) => self.structs.insert(i, s)
        }
    }

    pub fn append(&mut self, mut other: Self) {
        self.funcs.append(&mut other.funcs);
        self.free_funcs.append(&mut other.free_funcs);
        for s in other.structs {
            self.add_struct(s);
        }
    }
}