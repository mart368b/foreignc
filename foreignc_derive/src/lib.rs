extern crate proc_macro;

#[macro_use]
mod error;
mod generate;

use generate::*;
use foreignc_util::{throw_err, to_snake_case};

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
#[allow(unused_imports)]
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use quote::*;
use syn::*;

use core::convert::From;
use syn::spanned::Spanned;

#[cfg(feature = "template")]
mod template;
#[cfg(feature = "template")]
mod _template {
    pub use crate::template::*;
    pub use ffi_template::derived_input::*;
    pub use ffi_template::*;
}
#[cfg(feature = "template")]
use _template::*;

struct Items {
    pub impls: Option<ItemImpl>,
    pub items: Option<ItemFn>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![pub]) | input.peek(Token![crate]) | input.peek(Token![fn]) {
            Ok(Items {
                items: Some(input.parse()?),
                impls: None,
            })
        } else if input.peek(Token![impl]) {
            Ok(Items {
                items: None,
                impls: Some(input.parse()?),
            })
        } else {
            Ok(Items {
                impls: None,
                items: None,
            })
        }
    }
}

#[proc_macro_attribute]
pub fn with_abi(_attr: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let items: Items = throw_err!(parse(input));

    let ffi_impl = if let Some(item) = items.items {
        convert_item(item)
    } else if let Some(impls) = items.impls {
        let mut cimpls: TokenStream1 = impls.clone().into_token_stream().into();
        cimpls.extend::<TokenStream1>(convert_impls(impls));
        cimpls
    } else {
        throw_err!(msg: "Can only implement extern on impl of fn block")
    };

    ffi_impl
}

fn convert_impls(impls: ItemImpl) -> TokenStream1 {
    let span = impls.span().clone();
    let mut implementation = TokenStream1::new();

    #[cfg(feature = "template")]
    let mut implement = {
        let mut implement = RustStructure::default();
        if let Type::Path(ref p) = &*impls.self_ty {
            implement.self_ty = p.path.segments[0].ident.to_string();
        }
        implement
    };

    for item in impls.items {
        if let ImplItem::Method(item_fn) = item {
            let method_name = item_fn.sig.ident.clone();
            let item = throw_err!(convert_item_fn(&impls.self_ty, item_fn));

            #[cfg(feature = "template")]
            {
                let f = to_rust_func(&item, Some((&*impls.self_ty, &method_name)));
                implement.methods.push(throw_err!(f));
            }
            
            let extern_item_fn = throw_err!(to_extern_item_fn(item, Some((&*impls.self_ty, method_name))));
            implementation.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
            implementation.extend::<TokenStream1>(extern_item_fn.into_token_stream().into());
        }
    }

    #[cfg(feature = "template")]
    {
        throw_err!(
            add_to_path(implement)
                .map_err(|e| syn::Error::new(span, &e))
        );
    }

    implementation.into()
}

fn convert_item(item: ItemFn) -> TokenStream1 {
    let _span = item.span().clone();
    let mut implementation = TokenStream1::new();

    #[cfg(feature = "template")]
    {
        let func = throw_err!(to_rust_func(&item, None));
        throw_err!(
                add_to_path(func)
                .map_err(|e| syn::Error::new(_span, &e))
        );
    }

    implementation.extend::<TokenStream1>(item.to_token_stream().into());
    let mut extern_item_fn = throw_err!(to_extern_item_fn(item, None));
    extern_item_fn.sig.ident = Ident::new(&format!("{}_ffi", extern_item_fn.sig.ident), extern_item_fn.sig.ident.span().clone());
    let extern_stream: TokenStream2 = extern_item_fn.into_token_stream();
    implementation.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
    implementation.extend::<TokenStream1>(extern_stream.into());

    implementation
}

#[proc_macro]
pub fn generate_free_string(_item: TokenStream1) -> TokenStream1 {
    let mut output = TokenStream1::new();

    output.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
    output.extend::<TokenStream1>(
        quote!(
            pub extern "C" fn free_coption(ptr: *mut foreignc::c_void) {
                unsafe {
                    foreignc::free_libc(ptr);
                }
            }
        )
        .into(),
    );

    output.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
    output.extend::<TokenStream1>(
        quote!(
            pub extern "C" fn free_cresult(ptr: *mut foreignc::CResult<foreignc::c_void, foreignc::c_void>) {
                unsafe {
                    let res = &*ptr;
                    foreignc::free_libc(res.value);
                    foreignc::free_libc(ptr as *mut foreignc::c_void);
                };
            }
        )
        .into(),
    );

    output.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
    output.extend::<TokenStream1>(
        quote!(
            pub extern "C" fn free_string(ptr: *mut std::os::raw::c_char) {
                let _s = unsafe { foreignc::CString::from_raw(ptr) };
            }
        )
        .into(),
    );

    #[cfg(feature = "template")]
    {
        let free = RustFreeFunction {
            ty: RustTypes::String,
            func: RustFunction {
                name: "free_string".to_owned(),
                extern_name: "free_string".to_owned(),
                inputs: vec![RustArgument {
                    name: "ptr".to_owned(),
                    ty: RustTypes::String,
                }],
                output: None,
            },
        };

        throw_err!(
            add_to_path(free)
                .map_err(|e| syn::Error::new(Span::call_site(), &e))
        );
    }

    output
}

#[proc_macro_derive(Boxed)]
pub fn derive_boxed(input: TokenStream1) -> TokenStream1 {
    let item: Item = throw_err!(parse(input));
    let _span = item.span().clone();
    let name = match &item {
        Item::Enum(ref e) => &e.ident,
        Item::Struct(ref s) => &s.ident,
        _ => throw_err!(msg: "Cannot derive for that"),
    };

    let mut t: TokenStream1 = quote!(
        unsafe impl foreignc::IntoFFi<*mut #name> for #name {
            fn into_ffi(v: Self) ->  FFiResult<*mut #name> {
                Ok(unsafe { Box::into_raw(Box::new(v)) })
            }
        }
        unsafe impl foreignc::FromFFi<*mut #name> for &mut #name {
            fn from_ffi(ptr: *mut #name) -> foreignc::FFiResult<Self> {
                Ok(unsafe {
                    ptr
                        .as_mut()
                        .ok_or_else(|| FFiError::from("Recieved null pointer to #name"))?
                })
            }
        }
        unsafe impl foreignc::FromFFi<*mut #name> for &#name {
            fn from_ffi(ptr: *mut #name) -> foreignc::FFiResult<Self> {
                Ok(unsafe {
                    ptr
                        .as_ref()
                        .ok_or_else(|| FFiError::from("Recieved null pointer to #name"))?
                })
            }
        }
    )
    .into();

    let tt_name = Ident::new(
        &to_snake_case("free_".to_owned() + &name.to_string()),
        item.span(),
    );
    let tt: TokenStream1 = quote!(
        pub extern "C" fn #tt_name(ptr: *mut #name) {
            let _: Box<#name> = unsafe{ Box::from_raw(ptr) };
        }
    )
    .into();

    #[cfg(feature = "template")]
    {
        let free = RustFreeFunction {
            ty: RustTypes::Ptr(name.to_string()),
            func: RustFunction {
                name: tt_name.to_string(),
                extern_name: tt_name.to_string(),
                inputs: vec![RustArgument {
                    name: "ptr".to_owned(),
                    ty: RustTypes::Ptr(tt_name.to_string()),
                }],
                output: None,
            },
        };

        throw_err!(
            add_to_path(free)
                .map_err(|e| syn::Error::new(_span, &e))
        );

        let s = RustStructure{
            self_ty: name.to_string(),
            methods: Vec::new(),
            destructor: Some(tt_name.to_string()),
            ty: StructTypes::Boxed
        };
        throw_err!(
            add_to_path(s)
                .map_err(|e| syn::Error::new(_span, &e))
        );
    }

    t.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().expect("Failed to parse no_mangle"));
    t.extend(tt);

    t
}


#[proc_macro_derive(Json)]
pub fn derive_json(input: TokenStream1) -> TokenStream1 {
    let item: Item = throw_err!(parse(input));
    let _span = item.span().clone();
    let name = match &item {
        Item::Enum(ref e) => &e.ident,
        Item::Struct(ref s) => &s.ident,
        _ => throw_err!(msg: "Can only implement extern on impl of fn block"),
    };

    #[cfg(feature = "template")]
    {
        let s = RustStructure{
            self_ty: name.to_string(),
            methods: Vec::new(),
            destructor: Some("free_string".to_owned()),
            ty: StructTypes::Json
        };
        throw_err!(
            add_to_path(s)
                .map_err(|e| syn::Error::new(_span, &e))
        );
    }
    
    quote!(
        unsafe impl foreignc::FromFFi<*mut #name> for #name{
            fn from_ffi(p: *mut #name) -> foreignc::FFiResult<Self> {
                println!("from c_char");
                let s = foreignc::FromFFi::from_ffi(p as *mut std::os::raw::c_char);
                println!("--{:?}--", s);
                let json = serde_json::from_str(s?);
                println!("--{:?}--", json);
                Ok(json?)
            }
        }

        unsafe impl foreignc::IntoFFi<*mut #name> for &mut #name {
            fn into_ffi(v: Self) -> FFiResult<*mut #name> {
                let s = serde_json::to_string(v)?;
                Ok(foreignc::IntoFFi::into_ffi(s)? as *mut #name)
            }
        }

        unsafe impl foreignc::IntoFFi<*mut #name> for &#name {
            fn into_ffi(v: Self) -> FFiResult<*mut #name> {
                let s = serde_json::to_string(v)?;
                Ok(foreignc::IntoFFi::into_ffi(s)? as *mut #name)
            }
        }

        unsafe impl foreignc::IntoFFi<*mut #name> for #name {
            fn into_ffi(v: Self) -> FFiResult<*mut #name> {
                let s = serde_json::to_string(&v)?;
                Ok(foreignc::IntoFFi::into_ffi(s)? as *mut #name)
            }
        }
    )
    .into()
}