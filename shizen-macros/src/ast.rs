use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
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

pub enum Args {}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        match input.cursor().ident() {
            _ => Err(lookahead.error()),
        }
    }
}
