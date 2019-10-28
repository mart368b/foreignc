extern crate proc_macro;

mod func;
mod arguments;
mod generate;
mod generics;

use generics::*;
use generate::*;
use arguments::*;

use func::*;
use proc_macro_error::*;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

use syn::spanned::Spanned;
use core::convert::From;

static mut HAS_FREE_STRING: bool = false;
static mut HAS_TAKE_ERROR: bool = false;

#[proc_macro_attribute]
pub fn inspect(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let it: ItemFn = parse(item).unwrap();
    println!("{:#?}", it);
    it.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn implement_extern(_attr: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let items: Items = parse(input).unwrap();
    
    let mut ffi_impl = if let Some(item) = items.items {
        convert_items(item)
    }else if let Some(impls) = items.impls {
        convert_impls(impls)
    }else {
        abort!("Can only implement extern on impl of fn block")
    };

    unsafe{
        if !HAS_FREE_STRING {
            ffi_impl.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
            ffi_impl.extend::<TokenStream1>(generate_free_string().into());
            HAS_FREE_STRING = true;
        }
        if !HAS_TAKE_ERROR {
            ffi_impl.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
            ffi_impl.extend::<TokenStream1>(generate_take_error().into());
            HAS_TAKE_ERROR = true;
        }
    };

    ffi_impl
}

fn convert_impls(impls: ItemImpl ) -> TokenStream1 {
    let mut implementation = TokenStream1::new();

    let mut implement = Implement::default();
    if let Type::Path(ref p) = &*impls.self_ty {
        implement.self_ty = p.path.segments[0].ident.to_string();
    }

    implementation.extend::<TokenStream1>(impls.clone().into_token_stream().into());

    for item in impls.items {
        if let ImplItem::Method(item_fn) = item {
            let item = convert_item_fn(&impls.self_ty, item_fn);
            let (extern_item_fn, f) = to_extern_item_fn(item, &Vec::new(), Some(&*impls.self_ty));
            let extern_stream: TokenStream2 = extern_item_fn.into_token_stream();
            implementation.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
            implementation.extend::<TokenStream1>(extern_stream.into());
            implement.methods.push(f);
        }
    }

    implementation
}

fn convert_items(item: ItemFn ) -> TokenStream1 {
    let mut implementation = TokenStream1::new();

    implementation.extend::<TokenStream1>(item.clone().into_token_stream().into());

    let (extern_item_fn, f) = to_extern_item_fn(item, &Vec::new(), None);
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
        unsafe impl generator::FromFFi<*const std::os::raw::c_char> for #name{
            fn from_ffi(p: *const std::os::raw::c_char) -> Self {
                let s = generator::FromFFi::from_ffi(p);
                serde_json::from_str(s).unwrap()
            }
        }

        unsafe impl generator::IntoFFi<*mut std::os::raw::c_char> for #name {
            fn into_ffi(v: Self) -> *mut std::os::raw::c_char { 
                let s = serde_json::to_string(&v).unwrap();
                generator::IntoFFi::into_ffi(s)
            }
        }
    ).into()
}

fn generate_free_string() -> TokenStream2 {
    quote!(
        pub extern "C" fn free_string(ptr: *mut std::os::raw::c_char) {
            let _ = unsafe{ generator::CString::from_raw(ptr) };
        }
    )
}

fn generate_take_error() -> TokenStream2 {
    quote!(
        pub extern "C" fn last_error() -> *mut std::os::raw::c_char {
            if let Some(e) = generator::take_last_error() {
                generator::IntoFFi::into_ffi(format!("{}", e))
            }else {
                generator::IntoFFi::into_ffi("")
            }
        }
    )
}

fn to_snake_case(s: String) -> String {
    let mut ss = String::new();
    let mut require_space = false;
    for c in s.chars() {
        if c.is_uppercase() && require_space {
            ss.push(' ');
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

#[proc_macro_derive(BOXED)]
pub fn derive_boxed(input: TokenStream1) -> TokenStream1 {
    let item: Item = parse(input).unwrap();
    let name = match &item {
        Item::Enum(ref e) => &e.ident,
        Item::Struct(ref s) => &s.ident,
        _ => abort!("Cannot derive for that")
    };

    let mut t: TokenStream1 = quote!(
        unsafe impl<T> generator::IntoFFi<*mut T> for #name {
            fn into_ffi(v: Self) -> *mut T { 
                unsafe { std::mem::transmute(Box::new(v)) }
            }
        }
        unsafe impl<T> generator::FromFFi<*mut T> for &mut #name {
            fn from_ffi(ptr: *mut T) -> Self { 
                unsafe { &mut *(ptr as *mut #name) }
            }
        }
        unsafe impl<T> generator::FromFFi<*mut T> for &#name {
            fn from_ffi(ptr: *mut T) -> Self { 
                unsafe { &*(ptr as *mut #name) } 
            }
        }
    ).into();
    
    let tt_name = Ident::new(&to_snake_case("free_".to_owned() + &name.to_string()), item.span());
    let tt: TokenStream1 = quote!(
        pub extern "C" fn #tt_name(ptr: *mut std::ffi::c_void) {
            let _ = unsafe{ std::boxed::Box::from_raw(ptr as *mut #name) };
        }
    ).into();

    t.extend::<TokenStream1>("#[no_mangle]".parse::<TokenStream1>().unwrap());
    t.extend(tt);

    t
}