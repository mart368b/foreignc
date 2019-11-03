use std::path::Path;

pub trait IArgument<Ty>
where
    Ty: ToString,
{
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &Ty;
}

pub trait IFunction<Arg, Ty>
where
    Ty: ToString,
    Arg: IArgument<Ty>,
{
    fn get_name(&self) -> &str;
    fn get_ffi_name(&self) -> &str;
    fn get_args(&self) -> &Vec<Arg>;
    fn get_return(&self) -> Option<&Ty>;
}

pub trait IStructure<Func, Arg, Ty>
where
    Ty: ToString,
    Arg: IArgument<Ty>,
    Func: IFunction<Arg, Ty>,
{
    fn get_name(&self) -> &str;
    fn get_methods(&self) -> &Vec<Func>;
}

pub trait IGenerator<Struct, Func, Arg, Ty>
where
    Ty: ToString,
    Arg: IArgument<Ty>,
    Func: IFunction<Arg, Ty>,
    Struct: IStructure<Func, Arg, Ty>,
{
    fn write_to(&self, dir: &Path);
}

/*

pub struct OutputFile<Fmt, Func, Arg, Ty>
where
    Ty: ToString,
    Arg: Argument<Ty>,
    Func: Function<Arg, Ty>,
{
    file_name: String,
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
        let c = Tera::one_off(&template, &context, false);
        let mut f = File::create(p);
        f.write_all(&c.as_bytes());
    }
}
*/
