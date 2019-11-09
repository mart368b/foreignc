use super::arguments::*;
use super::error::DResult;
use proc_macro2::Span;
pub use ffi_template::{RustArgument, RustFunction, RustStructure, RustTypes};
use syn::*;

pub fn to_rust_func(
    item: &ItemFn,
    implm: Option<(&Type, &Ident)>,
) -> DResult<RustFunction> {
    let mut f = RustFunction::default();

    for ty in &item.sig.inputs {
        match ty {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(Span::call_site(), "Cannot have self in item fn").into());
            }
            FnArg::Typed(ref t) => {
                let tty = convert_to_rust_type(&t.ty)?;
                if let Pat::Ident(ref ident) = &*t.pat {
                    f.inputs.push(RustArgument {
                        name: ident.ident.to_string(),
                        ty: tty,
                    });
                }
            }
        }
    }

    if let Some((_, method_name)) = implm {
        f.name = method_name.to_string();
        f.extern_name = item.sig.ident.to_string();
    } else {
        f.name = item.sig.ident.to_string();
        f.extern_name = f.name.clone();
    }

    if let ReturnType::Type(_, ref ty) = item.sig.output {
        let tty = convert_to_rust_type(ty)?;
        f.output = Some(tty);
    };
    Ok(f)
}

pub fn convert_to_rust_type(ty: &Box<Type>) -> DResult<RustTypes> {
    match &**ty {
        Type::Reference(ref r) => convert_to_rust_type(&r.elem),
        Type::Ptr(_) => Ok(RustTypes::Ptr("c_void".to_owned())),
        Type::Path(ref path) => {
            let seg0 = &path.path.segments[0];
            let path_name = seg0.ident.to_string();
            if path_name == "Result" || path_name == "Option" {
                if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                    if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                        let t = Box::new(inner_ty.clone());
                        let inner = convert_to_rust_type(&t)?;
                        if path_name == "Result" {
                            Ok(RustTypes::Result(Box::new(inner)))
                        } else {
                            Ok(RustTypes::Option(Box::new(inner)))
                        }
                    } else {
                        return Err(syn::Error::new(Span::call_site(), "Result or option should not have lifetime").into());
                    }
                } else {
                    return Err(syn::Error::new(Span::call_site(), "Expected generic arguments after Result or Option").into());
                }
            } else {
                if path_name.ends_with("String") | path_name.ends_with("str") {
                    Ok(RustTypes::String)
                } else {
                    match path_name.as_str() {
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                        | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" | "bool"
                        | "char" => Ok(RustTypes::Primitive(path_name.as_str().to_owned())),
                        _ => Ok(RustTypes::Ptr(path_name.as_str().to_owned())),
                    }
                }
            }
        }
        _ => Ok(RustTypes::Ptr("c_void".to_owned())),
    }
}
