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
        match input.cursor().ident() {
            Some((id, _)) if id == "config" => Ok(Args::Config {
                config_token: input.parse()?,
                eq_token: input.parse()?,
                file_name: input.parse()?,
            }),
            _ => Err(lookahead.error()),
        }
    }
}
