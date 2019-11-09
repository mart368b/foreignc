use syn::parse::{Parse, ParseStream};
use syn::*;

pub struct Items {
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
