use proc_macro2::TokenStream;
#[cfg(feature = "template")]
use ffi_template::TemplateError;
use foreignc_err::impl_from;

#[derive(Debug)]
pub enum DeriveError {
    SynErr(syn::Error),
    #[cfg(feature = "template")]
    TemplateErr(TemplateError)
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

pub type DResult<T> = Result<T, DeriveError>;