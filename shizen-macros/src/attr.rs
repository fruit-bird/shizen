use plugin_config::Config;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Error, ItemFn, Result};

use crate::ast::*;

/// Still thinking of a good structure for the plugins,
/// maybe like Yew functional components?
///         
/// Turns out, Yew function components are actually converted into structs,
/// that's why you automagically get the CamelCase warning.
///
/// Either way, it seems like a good decision to convert the fn to a struct,
/// have the function args be the struct fields,
/// and have the body be the struct's impl a Plugin trait.
/// And have the Plugin trait handle the heavy stuff when
/// interfacing with VST hosts
/// This is, if I want to keep the functional component-like API
///
/// - [ ] Now I have to understand how to FFI with the C++ VST SDK
/// - [x] Later, add an optional arg that's like `#[shizen(config = "plugin.conf.toml")]`"
pub fn plugin_impl(parsed_args: PluginArgs, parsed_input: ItemFn) -> Result<TokenStream> {
    let PluginArgs { args } = parsed_args;

    for arg in args.iter() {
        match arg {
            Args::Config { file_name, .. } => impls::config_impl(file_name)?,
        }
    }

    // let ItemFn { ref mut sig, .. } = parsed_input;
    // let lifetime = syn::parse_quote!('a);
    // let const_gen = syn::parse_quote!(const CHANNELS: usize);
    // sig.generics.params.push(lifetime);
    // sig.generics.params.push(const_gen);
    // sig.output = syn::parse_quote!(-> shizen::prelude::PluginResult<shizen::prelude::AudioBuffer<'a, CHANNELS>>);

    // return Err(Error::new(Span::call_site(), format!("{:?}", file_name)));

    Ok(parsed_input.into_token_stream())
}

mod impls {
    use syn::LitStr;

    use super::*;

    pub fn config_impl(file_name: &LitStr) -> Result<()> {
        let file_name = file_name.value();
        // do something with the config data
        let _config = Config::from_toml(&file_name).map_err(comp_error)?;
        Ok(())
    }
}

fn comp_error(e: Box<dyn std::error::Error>) -> Error {
    Error::new(Span::call_site(), e)
}
