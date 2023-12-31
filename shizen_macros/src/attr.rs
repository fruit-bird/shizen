#![allow(dead_code)]

use plugin_config::PluginConfig;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Error, FnArg, ItemFn, Pat, PatIdent, PatType, Result, ReturnType};

const PLUGIN_SUFFIX: &str = "Plugin";
const RETURN_TYPE: &str = "PluginResult";
const RETURN_TYPE_INNER: &str = "AudioBuffer";

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
/// - Now I have to understand how to FFI with the C++ VST SDK
/// - Later, add an optional arg that's like `#[shizen(config = "plugin.conf.json")]`"
pub fn _plugin_impl(parsed_input: ItemFn) -> Result<TokenStream> {
    // commented out for now because of error in checker
    // let output = parsed_input.sig.output.clone();
    // let _return_ty = checkers::check_return_type(output)?;

    // keep this check for now, until things are changed into a struct
    // that impls Plugin
    let fn_ident = &parsed_input.sig.ident;
    checkers::check_fnident(fn_ident)?;

    let _config = PluginConfig::new("plugin.conf.json")
        .map_err(|error| Error::new(Span::call_site(), error))?;
    // return Err(Error::new(Span::call_site(), format!("{:?}", _config)));

    Ok(parsed_input.to_token_stream())
}

// Too many implicit things, I don't like it. Either improve or remove this
pub fn plugin_impl_alt(parsed_input: ItemFn) -> Result<TokenStream> {
    // let output = parsed_input.sig.output.clone();
    // let _return_ty = checkers::check_return_type(output)?;

    let struct_attrs = parsed_input.attrs;
    let struct_vis = parsed_input.vis;
    let struct_name = parsed_input.sig.ident;
    let struct_fields = parsed_input.sig.inputs.iter().skip(1).collect::<Vec<_>>(); // skip the AudioBuffer

    let struct_field_names = struct_fields.clone();
    let struct_field_names = struct_field_names.iter().filter_map(|field| match field {
        FnArg::Typed(PatType { pat, .. }) => match **pat {
            Pat::Ident(PatIdent { ref ident, .. }) => Some(ident),
            _ => None,
        },
        FnArg::Receiver(_) => None,
    });

    // let process_audio_block = parsed_input
    //     .block
    //     .stmts
    //     .iter()
    //     .map(|stmt| stmt.into_token_stream().args_to_fields(&struct_field_names));
    let process_audio_block = parsed_input.block.stmts;

    // give absolute paths for types and traits
    let tokens = quote! {
        #(#struct_attrs)*
        #struct_vis struct #struct_name {
            #(#struct_vis #struct_fields)*
        }

        impl #struct_name {
            pub fn new(#(#struct_fields)*) -> Self {
                Self { #(#struct_field_names)* }
            }
        }

        impl shizen::prelude::Plugin for #struct_name {
            fn initialize(&mut self, _context: &shizen::prelude::VST3Context) -> PluginResult<()> {
                todo!("some FFI stuff probably most definitely")
            }

            fn process_audio(&mut self, audio_buffer: shizen::prelude::AudioBuffer) -> PluginResult<shizen::prelude::AudioBuffer> {
                #(#process_audio_block)*
            }

            fn process_midi(&mut self, _midi_messages: shizen::prelude::MidiBuffer) -> PluginResult<()> {
                PluginResult::Ok(())
            }
        }
    };

    Ok(tokens)
}

// pub trait ArgsToFields {
//     fn args_to_fields(self, fn_args: &[&Ident]) -> TokenStream;
// }

// impl ArgsToFields for TokenStream {
//     fn args_to_fields(self, fn_args: &[&Ident]) -> TokenStream {
//         self.into_iter()
//             .map(|tt| match tt {
//                 TokenTree::Group(g) => {
//                     let delimiter = g.delimiter();
//                     let stream = g.stream().args_to_fields(fn_args);

//                     let group = Group::new(delimiter, stream);
//                     quote! { #group }
//                 }
//                 TokenTree::Ident(ref ident) if fn_args.contains(&ident) => quote! { self.#ident },
//                 _ => todo!(),
//             })
//             .collect()
//     }
// }

mod checkers {
    use super::*;
    use syn::{
        AngleBracketedGenericArguments, GenericArgument, PathArguments, PathSegment, Type, TypePath,
    };

    pub fn check_return_type(return_ty: ReturnType) -> Result<Type> {
        let return_ty = match return_ty {
            ReturnType::Default => {
                return Err(Error::new(
                    return_ty.span(),
                    "Plugins should return `PluginResult<AudioBuffer>`",
                ))
            }
            ReturnType::Type(_, ty) => *ty,
        };

        // doesn't work because is_ident needs the reciever to not have a path, and not have segments (ie. generics)
        let is_correct_type = |return_ty: &Type| match return_ty {
            Type::Path(TypePath { ref path, .. }) => {
                if let Some(PathSegment {
                    arguments:
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
                    ..
                }) = path.segments.first()
                {
                    match args.first() {
                        Some(GenericArgument::Type(Type::Path(TypePath { path, .. })))
                            if path.is_ident(RETURN_TYPE_INNER) =>
                        {
                            true
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        };

        match is_correct_type(&return_ty) {
            true => Ok(return_ty),
            false => Err(Error::new(
                return_ty.span(),
                "A plugin function should only return a `PluginResult<AudioBuffer>`",
            )),
        }
        // Ok(return_ty)
    }

    pub fn check_fnident(fn_ident: &Ident) -> Result<()> {
        let fn_ident_string = fn_ident.to_string();
        if !fn_ident_string.ends_with(PLUGIN_SUFFIX) {
            return Err(Error::new(
                fn_ident.span(),
                format!(
                    "Plugin names should be suffixed with `{1}`. Try to {}{1}",
                    fn_ident, PLUGIN_SUFFIX
                ),
            ));
        }

        // TODO: get rid of this check once functional components are implemented,
        //       becasue they will turn the fn to a struct, Rust will warn against
        //       lowercase idents
        if fn_ident_string.chars().next().unwrap().is_lowercase() {
            // SAFETY: safe unwrap because function ident has to have at least one char + Plugin
            let upper_first = (fn_ident_string.chars().next().unwrap() as u8 - 0x20) as char;
            return Err(Error::new(
                fn_ident.span(),
                format!(
                    "Plugin naming convention is CamelCase, try {}{}",
                    upper_first,
                    &fn_ident_string[1..]
                ),
            ));
        }

        Ok(())
    }
}
