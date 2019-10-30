use super::arguments::*;
use proc_macro_error::*;
use syn::*;
use core::convert::From;
pub use ffi_template::{RustFunction, RustTypes, Argument, ImplementBlock};

pub fn to_rust_func(item: &ItemFn, casts: &Vec<TypeCast>, implm: Option<(&Type, &Ident)>) -> RustFunction {
    let mut f = RustFunction::default();

    for ty in &item.sig.inputs {
        match ty {
            FnArg::Receiver(_) => {
                abort!("Cannot have self in item fn");
            },
            FnArg::Typed(ref t) => {
                let tty = convert_to_rust_type(&t.ty, &casts);
                if let Pat::Ident(ref ident) = &*t.pat {
                    f.inputs.push(Argument{
                        name: ident.ident.to_string(), 
                        ty: tty
                    });
                }
            }
        }
    };

    if let Some((_, method_name)) = implm {
        f.name = method_name.to_string();
        f.extern_name = item.sig.ident.to_string();
    }else {
        f.name = item.sig.ident.to_string();
        f.extern_name = f.name.clone();
    }

    if let ReturnType::Type(_, ref ty) = item.sig.output {
        let tty = convert_to_rust_type(ty, casts);
        f.output = Some(tty);
    };
    f
}

pub fn convert_to_rust_type(ty: &Box<Type>, casts: &Vec<TypeCast>) -> RustTypes {
    match &**ty {
        Type::Reference(ref r) => {
            convert_to_rust_type(&r.elem, casts)
        }
        Type::Path(ref path) => {
            let seg0 = &path.path.segments[0];
            let path_name = seg0.ident.to_string();
            if path_name == "Result" || path_name == "Option" {
                if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                    if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                        let t = Box::new(inner_ty.clone());
                        let inner = convert_to_rust_type(&t, casts);
                        if path_name == "Result" {RustTypes::Result(Box::new(inner))} else {RustTypes::Option(Box::new(inner))}
                    }else {
                        abort!("Result or option should not have lifetime")
                    }
                }else {
                    abort!("Expected generic arguments after Result or Option")
                }
            }else {
                if let Some(ref cast) = casts.iter().find(|c| c.ty0.to_string() == path_name) {
                    match cast.ty {
                        Types::JSON => RustTypes::Json(cast.ty0.to_string()),
                    }
                }else {
                    if path_name.ends_with("String") | path_name.ends_with("str") {
                        RustTypes::String
                    }else {
                        match path_name.as_str() {
                            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" 
                            | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" 
                            | "f32" | "f64"
                            | "bool" | "char" => RustTypes::Primitive(path_name.as_str().to_owned()),
                            _ => RustTypes::Ptr(path_name.as_str().to_owned()
                            )
                        }
                    }
                }
            }
        }
        Type::Ptr(_) => RustTypes::Ptr("c_void".to_owned()),
        _ => RustTypes::Ptr("c_void".to_owned())
    }
}
