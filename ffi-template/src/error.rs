use std::*;
use foreignc_util::impl_from;

#[derive(Debug)]
pub enum TemplateError {
    IoErr(io::Error),
    VarErr(env::VarError),
    SerdeErr(serde_json::error::Error),
    TeraErr(tera::Error)
}

impl_from! {
    io::Error => TemplateError::IoErr,
    env::VarError => TemplateError::VarErr,
    serde_json::error::Error => TemplateError::SerdeErr,
    tera::Error => TemplateError::TeraErr
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::IoErr(e) => e.fmt(f),
            TemplateError::VarErr(e) => e.fmt(f),
            TemplateError::SerdeErr(e) => e.fmt(f),
            TemplateError::TeraErr(e) => e.fmt(f)
        }
    }
}

pub type TResult<T> = Result<T, TemplateError>;