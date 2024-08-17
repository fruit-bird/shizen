use syn::{
    spanned::Spanned, Error, FnArg, Pat, PatIdent, PatType, Result, ReturnType, Signature, Type,
};

pub fn validate_sig_len(sig: &Signature) -> Result<()> {
    match sig.inputs.len() {
        0 => Err(Error::new(
            sig.inputs.span(),
            "Function must have at least one argument",
        )),
        1 | 2 => Ok(()),
        _ => Err(Error::new(
            sig.inputs.span(),
            "Function must have at most two arguments",
        )),
    }
}

pub fn extract_types_from_sig(sig: &Signature) -> Result<(&PatIdent, &Type, &Type)> {
    let (input_ident, input_ty) = sig
        .inputs
        .first()
        .and_then(|arg| match arg {
            FnArg::Typed(PatType { ty, pat, .. }) => match **pat {
                Pat::Ident(ref ident) => Some((ident, &**ty)),
                _ => None,
            },
            _ => None,
        })
        .ok_or(Error::new(sig.inputs.span(), "Invalid argument"))?;

    let output_ty = match sig.output {
        ReturnType::Type(_, ref ty) => &**ty,
        _ => return Err(Error::new(sig.output.span(), "Missing return type")),
    };

    Ok((input_ident, input_ty, output_ty))
}
