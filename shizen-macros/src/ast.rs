use derive_more::From;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Result, Token,
};

pub struct PluginArgs {
    pub args: Punctuated<Args, Token![,]>,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

#[derive(From)]
pub enum Args {
    Config {
        config_token: Ident,
        eq_token: Token![=],
        file_name: LitStr,
    },
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) && input.peek2(Token![=]) {
            Ok(Args::Config {
                config_token: input.parse()?,
                eq_token: input.parse()?,
                file_name: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
