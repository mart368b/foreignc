use proc_macro2::TokenStream;
#[cfg(feature = "template")]
use ffi_template::TemplateError;

#[derive(Debug)]
pub enum DeriveError {
    SynErr(syn::Error),
    #[cfg(feature = "template")]
    TemplateErr(TemplateError)
}

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

impl_from! {
    syn::Error => DeriveError::SynErr
}

#[cfg(feature = "template")]
impl_from! {
    TemplateError => DeriveError::TemplateErr
}

impl DeriveError {
    pub fn to_compile_error(self) -> TokenStream {
        match self {
            DeriveError::SynErr(e) => e.to_compile_error(),
            #[cfg(feature = "template")]
            DeriveError::TemplateErr(e) => syn::Error::new(proc_macro2::Span::call_site(), e).to_compile_error()
        }
    }
}

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
            Err(err) => return $crate::export::TokenStream::from(err.to_compile_error())
        }
    };
}



pub type DResult<T> = Result<T, DeriveError>;