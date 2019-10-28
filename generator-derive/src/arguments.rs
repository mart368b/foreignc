use syn::parse::{ParseStream, Parse};
use syn::*;

pub enum Types {
    JSON
}

pub struct TypeCast {
    pub ty0: Ident,
    pub ty1: Box<Type>,
    pub ty: Types
}

impl Parse for TypeCast {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty0: Ident = input.parse()?;
        let _as_token: Token![as] = input.parse()?;
        let ty1: Ident = input.parse()?;

        let (ty, tty) = match ty1.to_string().as_str() {
            "JSON" => Ok((parse_str("*const std::os::raw::c_char")?, Types::JSON)),
            _ =>  Err(input.error("Unexpected FFI type type"))
        }?;

        Ok(TypeCast {
            ty0,
            ty1: Box::new(ty),
            ty: tty
        })
    }
}

pub struct Items {
    pub impls: Option<ItemImpl>,
    pub items: Option<ItemFn>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![pub]) | input.peek(Token![crate]) | input.peek(Token![fn]) {
            Ok(Items {
                items: Some(input.parse()?),
                impls: None
            })
        }else if input.peek(Token![impl]) {
            Ok(Items {
                items: None,
                impls: Some(input.parse()?)
            })
        }else  {
            Ok(Items {
                impls: None,
                items: None
            })    
        }
    }
}