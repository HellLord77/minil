use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;

pub fn peel_option(ty: &Type) -> Option<&Type> {
    let type_path = if let Type::Path(type_path) = ty {
        type_path
    } else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Option" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(args) => args,
        PathArguments::Parenthesized(_) | PathArguments::None => return None,
    };

    let ty = if args.args.len() == 1 {
        args.args.last().unwrap()
    } else {
        return None;
    };

    if let GenericArgument::Type(ty) = ty {
        Some(ty)
    } else {
        None
    }
}

pub fn peel_result_ok(ty: &Type) -> Option<&Type> {
    let type_path = if let Type::Path(type_path) = ty {
        type_path
    } else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Result" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(args) => args,
        PathArguments::Parenthesized(_) | PathArguments::None => return None,
    };

    let ty = if args.args.len() == 2 {
        args.args.first().unwrap()
    } else {
        return None;
    };

    if let GenericArgument::Type(ty) = ty {
        Some(ty)
    } else {
        None
    }
}
