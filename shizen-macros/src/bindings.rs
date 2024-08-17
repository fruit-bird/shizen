//! this stuff should probably be in a separate crate, that gets called
//! in the build.rs (??) of the template project for a shizen vst
//!
//! still, make them here, then move them later

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

pub fn generate_bindings(parsed_input: &ItemFn) -> Result<TokenStream> {
    let ItemFn { sig: _, .. } = parsed_input;
    Ok(quote! {})
}

mod bindings {
    // generate the necessary interop bindings to be able to generate a vst for an example plugin
    // do not generate rust macros, just the code for the bindings with the vst3-sys crate
}
