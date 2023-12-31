mod attr;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn shizen(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);
    attr::plugin_impl_alt(parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
