use super::*;

use super::error::*;
use foreignc_util::to_snake_case;
use core::convert::From;
use quote::*;
use std::iter::{Extend, FromIterator};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use std::str::FromStr;
use core::borrow::Borrow;
use syn::*;

pub fn to_extern_item_fn(
    mut item: ItemFn,
    implm: Option<(&Type, Ident)>,
) -> DResult<ItemFn> {
    let mut itemc = item.clone();
    let identc = Ident::new(&format!("{}_ffi", itemc.sig.ident), item.sig.ident.span());
    itemc.sig.ident = identc.clone();

    let mut args: Vec<&Pat> = Vec::new();

    for ty in &mut item.sig.inputs {
        match ty {
            // Convert self type
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(Span::call_site(), "Cannot have self in item fn").into());
            }
            // Use arg type
            FnArg::Typed(ref mut t) => {
                // [some_name]: [&|&mut] [some_type]
                // Convert arguments from [&|&mut] to [*const|*mut]
                if let Pat::Ident(ref mut p) = &mut *t.pat {
                    p.mutability = None;
                }
                args.push(&*t.pat);
                let ty = convert_to_ptr(t.ty.as_ref())?;
                t.ty = Box::new(parse(ty)?);
            }
        }
    }

    if let ReturnType::Type(_, ref mut ty) = item.sig.output {
        let mut out = TokenStream1::from_str("*mut foreignc::CResult<")?;
        out.extend(convert_to_ptr(ty.as_ref())?);
        out.extend(TokenStream1::from_str(", std::os::raw::c_char>"));
        *ty = Box::new(parse(out)?);
    }else {
        item.sig.output = ReturnType::Type(Token![->](itemc.span().clone()), parse_str("*mut foreignc::CResult<(), std::os::raw::c_char>")?);
    }

    Ok(ItemFn {
        block: Box::new(
            parse(if let Some((caller, method_name)) = implm {
                quote!(
                    {
                        unsafe {
                            let v = || -> foreignc::FFiResult<_> {
                                Ok(
                                    foreignc::IntoFFi::into_ffi(
                                        #caller::#method_name(#(
                                            foreignc::FromFFi::from_ffi(#args)?
                                        ),*)
                                    )?
                                )
                            }();
                            foreignc::FFiResultWrap::from(v).into()
                        }
                    }
                )
                .into()
            } else {
                quote!(
                    {
                        #itemc
                        unsafe {
                            let v = || -> foreignc::FFiResult<_> {
                                Ok(
                                    foreignc::IntoFFi::into_ffi(
                                        #identc(#(
                                            foreignc::FromFFi::from_ffi(#args)?
                                        ),*)
                                    )?
                                )
                            }();
                            foreignc::FFiResultWrap::from(v).into()
                        }
                    }
                )
                .into()
            })?
            ,
        ),
        vis: VisPublic {
            pub_token: Token![pub](item.sig.span()),
        }
        .into(),
        attrs: Vec::new(),
        sig: Signature {
            abi: Some(Abi {
                extern_token: Token![extern](item.sig.span()),
                name: Some(LitStr::new("C", item.sig.span())),
            }),
            ..item.sig
        },
    })
}

pub fn convert_item_fn(self_ty: &Box<Type>, item_fn: ImplItemMethod) -> DResult<ItemFn> {
    let mut inputs = Vec::new();
    for i in &item_fn.sig.inputs {
        let p_ty = if let FnArg::Receiver(r) = i {
            PatType {
                attrs: Vec::new(),
                pat: Box::new(
                    PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: r.mutability,
                        ident: Ident::new("this", r.span()),
                        subpat: None,
                    }
                    .into(),
                ),
                colon_token: Token![:](r.span()),
                ty: {
                    if r.reference.is_some() {
                        Box::new(
                            TypeReference {
                                and_token: Token![&](r.span()),
                                lifetime: None,
                                mutability: r.mutability,
                                elem: self_ty.clone(),
                            }
                            .into(),
                        )
                    } else {
                        self_ty.clone()
                    }
                },
            }
            .into()
        } else {
            i.clone()
        };
        inputs.push(p_ty);
    }
    Ok(ItemFn {
        vis: item_fn.vis,
        attrs: item_fn.attrs,
        sig: Signature {
            inputs: Punctuated::from_iter(inputs.into_iter()),
            ident: Ident::new(
                &to_snake_case(format!(
                    "{}{}",
                    &item_fn.sig.ident,
                    if let Type::Path(ref p) = &*self_ty.clone() {
                        p.path.segments[0].ident.to_string()
                    } else {
                        return Err(syn::Error::new(Span::call_site(), "Failed to get self type name").into());
                    }
                )),
                item_fn.sig.ident.span(),
            ),
            ..item_fn.sig
        },
        block: Box::new(item_fn.block),
    })
}

pub fn convert_to_ptr<T>(ref_ty: T) -> DResult<TokenStream1> 
where
    T: Borrow<Type>
{
    let ty = ref_ty.borrow();
    match ty {
        Type::Reference(ref r) => convert_to_ptr(r.elem.as_ref()),
        Type::Path(ref path) => {
            let seg0 = &path.path.segments[0];
            let path_name = seg0.ident.to_string();
            match path_name.as_str() {
                "Result" => {
                    if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                        if inner.args.len() != 2 {
                            return Err(syn::Error::new(inner.span().clone(), "Result should not have lifetime").into());
                        }
                        let s0 = if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                            convert_to_ptr(inner_ty)?
                        } else {
                            return Err(syn::Error::new(inner.args[0].span().clone(), "Result should not have lifetime").into());
                        };

                        let s1 = if let GenericArgument::Type(ref inner_ty) = inner.args[1] {
                            convert_to_ptr(inner_ty)?
                        } else {
                            return Err(syn::Error::new(inner.args[1].span().clone(), "Result should not have lifetime").into());
                        };
                        
                        Ok(TokenStream1::from_str(&format!("*mut CResult<*mut {}, *mut {}>", s0.to_string(), s1.to_string()))?)
                    } else {
                        return Err(syn::Error::new(seg0.arguments.span().clone(), "Expected generic arguments after Result or Option").into());
                    }
                }
                "Option" => {
                    if let PathArguments::AngleBracketed(ref inner) = seg0.arguments {
                        if let GenericArgument::Type(ref inner_ty) = inner.args[0] {
                            let inner = convert_to_ptr(inner_ty)?;
                            let mut stream = TokenStream1::from_str("*mut ")?;
                            stream.extend(inner);
                            Ok(stream)
                        } else {
                            return Err(syn::Error::new(inner.args[1].span().clone(), "Option should not have lifetime").into());
                        }
                    } else {
                        return Err(syn::Error::new(seg0.arguments.span().clone(), "Expected generic arguments after Result or Option").into());
                    }
                }
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" | "bool"
                | "char" => Ok(ty.to_token_stream().into()),
                "String" | "str" => Ok(TokenStream1::from_str("*mut std::os::raw::c_char")?),
                _ => Ok(TokenStream1::from_str(&format!("*mut {}", path_name))?),
            }
        }
        Type::Ptr(_) => Ok(ty.to_token_stream().into()),
        _ => {
            let mut stream = TokenStream1::from_str("*mut ")?;
            stream.extend::<TokenStream1>(ty.to_token_stream().into());
            Ok(stream)
        }
    }
}
