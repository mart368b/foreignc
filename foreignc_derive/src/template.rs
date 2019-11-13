use super::error::DResult;
use proc_macro2::Span;
pub use ffi_template::{RustArgument, RustFunction, RustStructure, RustTypes};
use syn::*;
use quote::ToTokens;
use core::borrow::Borrow;

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
                let tty = convert_to_rust_type(t.ty.as_ref())?;
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
        let tty = convert_to_rust_type(ty.as_ref())?;
        f.output = Some(tty);
    };
    Ok(f)
}

pub fn convert_to_rust_type<T>(ref_ty: T) -> DResult<RustTypes> 
where
    T: Borrow<Type>
{
    let ty = ref_ty.borrow();
    match ty {
        Type::Reference(ref r) => convert_to_rust_type(r.elem.as_ref()),
        Type::Path(ref path) => {
            let seg0 = &path.path.segments[0];
            let path_name = seg0.ident.to_string();
            match path_name.as_str() {
                "Result" => unimplemented!(),
                "Option" => {
                    if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                        if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                            Ok(RustTypes::Option(Box::new(convert_to_rust_type(inner_ty)?)))
                        } else {
                            return Err(syn::Error::new(Span::call_site(), "Result or option should not have lifetime").into());
                        }
                    } else {
                        return Err(syn::Error::new(Span::call_site(), "Expected generic arguments after Result or Option").into());
                    }
                }
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" | "bool"
                | "char" => Ok(RustTypes::Primitive(path_name.to_owned())),
                "String" | "str" => Ok(RustTypes::String),
                _ => Ok(RustTypes::Ptr(path_name.to_owned())),
            }
        }
        Type::Ptr(v) => Ok(RustTypes::Ptr(v.to_token_stream().to_string())),
        _ => Ok(RustTypes::Ptr("c_void".to_owned())),
    }
}