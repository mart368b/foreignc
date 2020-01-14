#[cfg(feature = "template")]
use foreignc_template::TemplateError;
use foreignc_util::impl_from;
use proc_macro2::TokenStream;

#[derive(Debug)]
pub enum DeriveError {
    SynErr(syn::Error),
    LexErr(proc_macro::LexError),
    #[cfg(feature = "template")]
    TemplateErr(TemplateError),
}

impl_from! {
    syn::Error => DeriveError::SynErr,
    proc_macro::LexError => DeriveError::LexErr
}

#[cfg(feature = "template")]
impl_from! {
    TemplateError => DeriveError::TemplateErr
}

impl DeriveError {
    pub fn to_compile_error(self) -> TokenStream {
        match self {
            DeriveError::SynErr(e) => e.to_compile_error(),
            DeriveError::LexErr(e) => {
                syn::Error::new(proc_macro2::Span::call_site(), format!("{:?}", e))
                    .to_compile_error()
            }
            #[cfg(feature = "template")]
            DeriveError::TemplateErr(e) => {
                syn::Error::new(proc_macro2::Span::call_site(), e).to_compile_error()
            }
        }
    }
}

pub type DResult<T> = Result<T, DeriveError>;
