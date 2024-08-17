mod ast;
mod attr;
mod bindings;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_args = parse_macro_input!(args as ast::PluginArgs);
    let parsed_input = parse_macro_input!(input as ItemFn);

    attr::plugin_impl(parsed_args, parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
