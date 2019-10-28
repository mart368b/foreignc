
use super::arguments::*;
use super::func::*;

use proc_macro_error::*;
use quote::*;
use syn::*;
use syn::spanned::Spanned;
use std::iter::{Extend, FromIterator};
use syn::punctuated::Punctuated;
use core::convert::From;

pub fn to_extern_item_fn(mut item: ItemFn, casts: &Vec<TypeCast>, implm: Option<&Type>) -> (ItemFn, Function) {
    let mut args: Vec<&Pat> = Vec::new();
    let item_name = &item.sig.ident;
    let mut f = Function::default();
    f.name = item_name.to_string();

    for ty in &mut item.sig.inputs {
        match ty {
            // Convert self type
            FnArg::Receiver(_) => {
                abort!("Cannot have self in item fn");
            },
            // Use arg type
            FnArg::Typed(ref mut t) => {
                // [some_name]: [&|&mut] [some_type]
                // Convert arguments from [&|&mut] to [*const|*mut]
                if let Pat::Ident(ref mut p) = &mut *t.pat {
                    p.mutability = None;
                }
                args.push(&*t.pat);
                let (ty, tty) = convert_to_ptr(&t.ty, &casts);
                t.ty = ty;
                if let Pat::Ident(ref ident) = &*t.pat {
                    f.inputs.push((ident.ident.to_string(), tty));
                }
            }
        }
    };

    if let ReturnType::Type(_, ref mut ty) = item.sig.output {
        let (nty, tty) = convert_to_ptr(ty, casts);
        *ty = nty;
        f.output = Some(tty);
        if let Type::Ptr(ref mut ptr) = &mut **ty {
            ptr.mutability = Some(Token![mut](ptr.span()));
            ptr.const_token = None;
        }
    };
    
    let func_name = item.sig.ident.to_string() + &if let Some(i) = &implm {
        if let Type::Path(path) = i {
            format!("_{}_ffi", path.path.segments[0].ident.to_string())
        }else {
            abort!("Failed")
        }
    }else {
        "_ffi".to_owned()
    };
    f.extern_name = format!("{}", func_name);

    let new_item = ItemFn {
        block: Box::new(
            parse(
                if let Some(caller) = implm {
                    quote!(
                        {
                            unsafe {
                                generator::IntoFFi::into_ffi(
                                    #caller::#item_name(#(
                                        generator::FromFFi::from_ffi(#args)
                                    ),*)
                                )
                            }
                        }
                    ).into()
                }else {
                    quote!(
                        {
                            unsafe {
                                generator::IntoFFi::into_ffi(
                                    #item_name(#(
                                        generator::FromFFi::from_ffi(#args)
                                    ),*)
                                )
                            }
                        }
                    ).into()
                }
            ).unwrap()
        ),
        vis: VisPublic{
            pub_token: Token![pub](item.sig.span())
        }.into(),
        attrs: Vec::new(),
        sig: Signature {
            abi:Some(Abi {
                extern_token: Token![extern](item.sig.span()),
                name: Some(LitStr::new("C", item.sig.span()))
            }),
            ident: Ident::new(&func_name, item.sig.ident.span()),
            .. item.sig.clone()
        },
    };
    (new_item, f)
}

pub fn convert_item_fn(self_ty: &Box<Type>, item_fn: ImplItemMethod) -> ItemFn {
    let mut inputs = Vec::new();
    for i in &item_fn.sig.inputs {
        let p_ty = if let FnArg::Receiver(r) = i {
            PatType {
                attrs: Vec::new(),
                pat: Box::new(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: r.mutability,
                    ident: Ident::new("this", r.span()),
                    subpat: None,
                }.into()),
                colon_token: Token![:](r.span()),
                    ty: {
                        if r.reference.is_some() {
                            Box::new(TypeReference {
                                and_token: Token![&](r.span()),
                                lifetime: None,
                                mutability: r.mutability,
                                elem: self_ty.clone()
                            }.into())
                        }else {
                            self_ty.clone()
                        }
                    }
            }.into()
        }else {
            i.clone()
        };
        inputs.push(p_ty);
    }
    ItemFn {
        vis: item_fn.vis,
        attrs: item_fn.attrs,
        sig: Signature {
            inputs: Punctuated::from_iter(inputs.into_iter()),
            ..item_fn.sig
        },
        block: Box::new(item_fn.block)
    }
}

pub fn convert_to_ptr(ty: &Box<Type>, casts: &Vec<TypeCast>) -> (Box<Type>, Arg) {
    match &**ty {
        Type::Reference(ref r) => {
            convert_to_ptr(&r.elem, casts)
        }
        Type::Path(ref path) => {
            let seg0 = &path.path.segments[0];
            let path_name = seg0.ident.to_string();
            if path_name == "Result" || path_name == "Option" {
                if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                    if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                        let t = Box::new(inner_ty.clone());
                        let (ty, a) = convert_to_ptr(&t, casts);
                        (ty, if path_name == "Result" {Arg::Result(Box::new(a))} else {Arg::Option(Box::new(a))})
                    }else {
                        abort!("Result or option should not have lifetime")
                    }
                }else {
                    abort!("Expected generic arguments after Result or Option")
                }
            }else {
                if let Some(ref cast) = casts.iter().find(|c| c.ty0.to_string() == path_name) {
                    match cast.ty {
                        Types::JSON => (cast.ty1.clone(), Arg::JSON(cast.ty0.to_string())),
                    }
                }else {
                    if path_name.ends_with("String") | path_name.ends_with("str") {
                        (Box::new(parse_str("*const std::os::raw::c_char").unwrap()), Arg::String)
                    }else {
                        match path_name.as_str() {
                            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" 
                            | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" 
                            | "f32" | "f64"
                            | "bool" | "char" => (ty.clone(), Arg::Pimitive(path_name.as_str().to_owned())),
                            _ => (
                                Box::new(TypePtr {
                                    star_token: Token![*](ty.span()),
                                    const_token: None,
                                    mutability: Some(Token![mut](ty.span())),
                                    elem: Box::new(parse_str("std::ffi::c_void").unwrap())
                                }.into()), 
                                Arg::Ptr(path_name.as_str().to_owned())
                            )
                        }
                    }
                }
            }
        }
        Type::Ptr(_) => (ty.clone(), Arg::Ptr("Raw".to_owned())),
        _ => {
            (Box::new(TypePtr {
                star_token: Token![*](ty.span()),
                const_token: None,
                mutability: Some(Token![mut](ty.span())),
                elem: ty.clone()
            }.into()), Arg::Ptr("Unknown".to_owned()))
        }
    }
}