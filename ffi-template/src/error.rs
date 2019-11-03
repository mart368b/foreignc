use std::*;

#[derive(Debug)]
pub enum TemplateError {
    IoErr(io::Error),
    VarErr(env::VarError),
    SerdeErr(serde_json::error::Error)
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
    io::Error => TemplateError::IoErr,
    env::VarError => TemplateError::VarErr,
    serde_json::error::Error => TemplateError::SerdeErr
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::IoErr(e) => e.fmt(f),
            TemplateError::VarErr(e) => e.fmt(f),
            TemplateError::SerdeErr(e) => e.fmt(f),
        }
    }
}

pub type TResult<T> = Result<T, TemplateError>;