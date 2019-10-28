use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use tera::{Context, Tera};
use std::marker::PhantomData;
use std::default::Default;

pub trait Argument<Ty> 
where
    Ty: ToString
{
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &Ty;
}

pub trait Function<Arg, Ty>
where 
    Ty: ToString,
    Arg: Argument<Ty>
{
    fn get_name(&self) -> &str;
    fn get_args(&self) -> &Vec<Arg>;
}

pub trait Format<Func, Arg, Ty>
where 
    Ty: ToString,
    Arg: Argument<Ty>,
    Func: Function<Arg, Ty>
{
    fn get_file_name(package_name: &str) -> String;
    fn get_template() -> String;
    fn load_function(&mut self, func: Func);
    fn get_context(&self) -> Context;
}

pub struct OutputFile<Fmt, Func, Arg, Ty>
where
    Ty: ToString,
    Arg: Argument<Ty>,
    Func: Function<Arg, Ty>,
    Fmt: Format<Func, Arg, Ty> + Default,
{
    file_name: String,
    format: Fmt,
    func: PhantomData<Func>,
    arg: PhantomData<Arg>,
    ty: PhantomData<Ty>
}

impl<Fmt, Func, Arg, Ty> OutputFile<Fmt, Func, Arg, Ty>
where
    Ty: ToString,
    Arg: Argument<Ty>,
    Func: Function<Arg, Ty>,
    Fmt: Format<Func, Arg, Ty> + Default,
{
    pub fn new() -> OutputFile<Fmt, Func, Arg, Ty> {
        let file_name = Fmt::get_file_name(env!("CARGO_PKG_NAME"));

        OutputFile {
            file_name: file_name,
            format: Fmt::default(),
            func: PhantomData,
            arg: PhantomData,
            ty: PhantomData
        }
    }

    pub fn load_function(&mut self, func: Func) {
        self.format.load_function(func);
    }

    pub fn render(&self) {
        let p = Path::new(&self.file_name);
        let template = Fmt::get_template();
        let context = self.format.get_context();
        let c = Tera::one_off(&template, &context, false).unwrap();
        let mut f = File::create(p).unwrap();
        f.write_all(&c.as_bytes()).unwrap();
    }
}