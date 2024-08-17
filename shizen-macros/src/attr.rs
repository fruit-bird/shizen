use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

use crate::ast::*;
use crate::bindings;
use crate::utils::*;

pub fn plugin_impl(_parsed_args: PluginArgs, parsed_input: ItemFn) -> Result<TokenStream> {
    let ItemFn {
        vis,
        sig,
        block,
        attrs,
    } = &parsed_input;

    validate_sig_len(&sig)?;

    let plugin_name = &sig.ident;
    let (input_ident, input_ty, output_ty) = extract_types_from_sig(&sig)?;

    let _bindings = bindings::generate_bindings(&parsed_input)?;

    Ok(quote! {
        #(#attrs)*
        #vis struct #plugin_name;

        // maybe turn this into a func that takes both audio and midi buffers
        // and remove the associated types
        //
        // and then ill have to parse the function body to see
        // if it uses midi or audio or both
        impl shizen_buffers::prelude::Plugin for #plugin_name {
            type InputBuffer = #input_ty;
            type OutputBuffer = #output_ty;

            fn process(#input_ident: #input_ty) -> #output_ty {
                #block
            }
        }
    })
}
