extern crate proc_macro;

#[cfg(feature = "template")]
mod template;
#[cfg(feature = "template")]
use template::*;
#[cfg(feature = "template")]
use ffi_template::derived_input::{add_func, add_impl};

mod arguments;
mod generate;

use generate::*;
use arguments::*;

use proc_macro_error::*;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

use syn::spanned::Spanned;
use core::convert::From;

#[proc_macro_attribute]
pub fn inspect(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let it: ItemFn = parse(item).unwrap();
    println!("{:#?}", it);
    it.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn wrap_extern(attr: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let parser = Punctuated::<TypeCast, Token![,]>::parse_terminated;
    let casts: Vec<TypeCast> = parser.parse(attr)
    .unwrap()
    .into_iter()
    .collect();
    let items: Items = parse(input).unwrap();
    
    let ffi_impl = if let Some(item) = items.items {
        convert_items(item, casts)
    }else if let Some(impls) = items.impls {
        let mut cimpls: TokenStream1 = impls.clone().into_token_stream().into();
        cimpls.extend::<TokenStream1>( convert_impls(impls, casts));
        cimpls
    }else {
        abort!("Can only implement extern on impl of fn block")
    };

    ffi_impl
}

fn convert_impls(impls: ItemImpl, casts: Vec<TypeCast> ) -> TokenStream1 {
    let mut implementation = TokenStream1::new();

    #[cfg(feature = "template")]
    let mut implement = {
        let mut implement = ImplementBlock::default();
        if let Type::Path(ref p) = &*impls.self_ty {
            implement.self_ty = p.path.segments[0].ident.to_string();
        }
        implement
    };
    
    for item in impls.items {
        if let ImplItem::Method(item_fn) = item {
            let method_name = item_fn.sig.ident.clone();
            let item = convert_item_fn(&impls.self_ty, item_fn);

            #[cfg(feature = "template")]
            {
                let f = to_rust_func(&item, &casts, Some((&*impls.self_ty, &method_name)));
                implement.methods.push(f);
            }

            let extern_item_fn = to_extern_item_fn(item, &casts, Some((&*impls.self_ty, method_name)));
            let extern_stream: TokenStream2 = extern_item_fn.into_token_stream();
            implementation.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
            implementation.extend::<TokenStream1>(extern_stream.into());
        }
    }

    #[cfg(feature = "template")]
    {
        add_impl(implement);
    }

    implementation
}

fn convert_items(item: ItemFn, casts: Vec<TypeCast> ) -> TokenStream1 {
    let mut implementation = TokenStream1::new();

    #[cfg(feature = "template")]
    {
        let func = to_rust_func(&item, &casts, None);
        add_func(func);
    }

    let extern_item_fn = to_extern_item_fn(item, &casts, None);
    let extern_stream: TokenStream2 = extern_item_fn.into_token_stream();
    implementation.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
    implementation.extend::<TokenStream1>(extern_stream.into());


    implementation
}

#[proc_macro_derive(Json)]
pub fn derive_json(input: TokenStream1) -> TokenStream1 {
    let item: Item = parse(input).unwrap();
    let name = match &item {
        Item::Enum(ref e) => &e.ident,
        Item::Struct(ref s) => &s.ident,
        _ => abort!("Cannot derive for that")
    };

    quote!(
        unsafe impl foreignc::FromFFi<*const std::os::raw::c_char> for #name{
            fn from_ffi(p: *const std::os::raw::c_char) -> Self {
                let s = foreignc::FromFFi::from_ffi(p);
                serde_json::from_str(s).unwrap()
            }
        }

        unsafe impl foreignc::IntoFFi<*mut std::os::raw::c_char> for &#name {
            fn into_ffi(v: Self) -> *mut std::os::raw::c_char { 
                let s = serde_json::to_string(v).unwrap();
                foreignc::IntoFFi::into_ffi(s)
            }
        }

        unsafe impl foreignc::IntoFFi<*mut std::os::raw::c_char> for #name {
            fn into_ffi(v: Self) -> *mut std::os::raw::c_char { 
                let s = serde_json::to_string(&v).unwrap();
                foreignc::IntoFFi::into_ffi(s)
            }
        }
        
    ).into()
}

#[proc_macro]
pub fn generate_free_string(_item: TokenStream1) -> TokenStream1 {
    let mut output = TokenStream1::new();

    output.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
    output.extend::<TokenStream1>(quote! (
        pub extern "C" fn free_string(ptr: *mut std::os::raw::c_char) {
            let _ = unsafe{ foreignc::CString::from_raw(ptr) };
        }
    ).into());

    output
}

#[proc_macro]
pub fn generate_last_error(_item: TokenStream1) -> TokenStream1 {
    let mut output = TokenStream1::new();

    output.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
    output.extend::<TokenStream1>(quote! (
        pub extern "C" fn last_error() -> *mut std::os::raw::c_char {
            if let Some(e) = foreignc::take_last_error() {
                foreignc::IntoFFi::into_ffi(format!("{}", e))
            }else {
                foreignc::IntoFFi::into_ffi("")
            }
        }
    ).into());

    output
}

fn to_snake_case(s: String) -> String {
    let mut ss = String::new();
    let mut require_space = false;
    for c in s.chars() {
        if c.is_uppercase() && require_space {
            ss.push('_');
        }
        if c.is_lowercase() {
            require_space = true;
        }else  if !c.is_alphabetic() && !c.is_numeric() {
            require_space = false;
        }
        ss.push(c.to_lowercase().next().unwrap());
    }

    ss
}

#[proc_macro_derive(Boxed)]
pub fn derive_boxed(input: TokenStream1) -> TokenStream1 {
    let item: Item = parse(input).unwrap();
    let name = match &item {
        Item::Enum(ref e) => &e.ident,
        Item::Struct(ref s) => &s.ident,
        _ => abort!("Cannot derive for that")
    };

    let mut t: TokenStream1 = quote!(
        unsafe impl foreignc::IntoFFi<*mut std::ffi::c_void> for #name {
            fn into_ffi(v: Self) -> *mut std::ffi::c_void { 
                unsafe { Box::into_raw(Box::new(v)) as *mut std::ffi::c_void }
            }
        }
        unsafe impl foreignc::FromFFi<*mut std::ffi::c_void> for &mut #name {
            fn from_ffi(ptr: *mut std::ffi::c_void) -> Self { 
                unsafe { &mut *(ptr as *mut #name) }
            }
        }
        unsafe impl foreignc::FromFFi<*mut std::ffi::c_void> for &#name {
            fn from_ffi(ptr: *mut std::ffi::c_void) -> Self { 
                unsafe { &*(ptr as *mut #name) } 
            }
        }
        /*
        Unsafe as it frees the memeory silently
        unsafe impl foreignc::FromFFi<*mut std::ffi::c_void> for #name {
            fn from_ffi(ptr: *mut std::ffi::c_void) -> Self { 
                let b: Box<#name> = unsafe{ Box::from_raw(ptr as *mut #name) };
                *b
            }
        }
        */
    ).into();
    
    let tt_name = Ident::new(&to_snake_case("free_".to_owned() + &name.to_string()), item.span());
    let tt: TokenStream1 = quote!(
        pub extern "C" fn #tt_name(ptr: *mut std::ffi::c_void) {
            let _: Box<#name> = unsafe{ Box::from_raw(ptr as *mut #name) };
        }
    ).into();

    t.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
    t.extend(tt);

    t
}