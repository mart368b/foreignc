
#[macro_export]
macro_rules! impl_from {
    ($($err:path => $ty:ident::$varient:ident),*) => {
        $(
            impl From<$err> for $ty {
                fn from(e: $err) -> $ty {
                    $ty::$varient(e)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! throw_err {
    (msg: $p:expr) => {
        throw_err!(span: proc_macro2::Span::call_site(), msg: $p)   
    };
    (span: $s:expr, msg: $p:expr) => {
        throw_err!(Err(syn::Error::new($s, $p)))   
    };
    ($v:expr) => {
        match $v {
            Ok(data) => data,
            Err(err) => return proc_macro::TokenStream::from(err.to_compile_error())
        }
    };
}