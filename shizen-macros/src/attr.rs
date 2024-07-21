use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemFn, Result};

use crate::ast::*;

pub fn plugin_impl(parsed_args: PluginArgs, parsed_input: ItemFn) -> Result<TokenStream> {
    let PluginArgs { args } = parsed_args;
    for arg in args.iter() {
        match arg {
            _ => return Err(Error::new(Span::call_site(), "Invalid argument")),
        }
    }

    // return Err(Error::new(Span::call_site(), format!("{:?}", )));
    Ok(quote! {
        #[allow(non_snake_case)]
        #parsed_input
    })
}
