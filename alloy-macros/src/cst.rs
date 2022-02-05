use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, ItemEnum, ItemStruct, Path,
    PathArguments, Type, TypePath,
};

// Strip the `CST` suffix from the given identifier if it exists, otherwise
// add `AST` suffix.
fn get_ast_ident(ident: &Ident) -> Ident {
    let cst = ident.to_string();
    let ast = if let Some(s) = cst.strip_suffix("CST") {
        s.to_string()
    } else {
        format!("{cst}AST")
    };
    Ident::new(&ast, ident.span())
}

// Check if field has `#[space]`
fn is_space(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "space")
}

/// Check whether given path is part of other qualified path.
fn compare_path(tp: &Path, other: Vec<&'static str>) -> bool {
    if tp.leading_colon.is_some() {
        return false;
    }
    for (segment, s) in tp.segments.iter().rev().zip(other.iter().rev()) {
        if segment.ident != s {
            return false;
        }
    }
    true
}

fn is_spanned(ty: &Type) -> bool {
    if let Type::Path(TypePath { qself, path }) = ty {
        if qself.is_some() {
            return false;
        }
        compare_path(path, vec!["alloy", "parser", "Spanned"])
            || compare_path(path, vec!["crate", "parser", "Spanned"])
    } else {
        false
    }
}

fn map_field(mut field: Field) -> Field {
    field.ty = extract_from_spanned(field.ty);
    field
}

fn extract_from_spanned(ty: Type) -> Type {
    let segment = if let Type::Path(TypePath { qself: _, path }) = ty {
        path.segments.into_iter().last().unwrap()
    } else {
        unreachable!()
    };
    if let PathArguments::AngleBracketed(args) = segment.arguments {
        extract_single_generic_type(args)
    } else {
        panic!("`Spanned<T>` takes only a single type argument.")
    }
}

fn extract_single_generic_type(args: AngleBracketedGenericArguments) -> Type {
    let mut args = args.args.into_iter();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        panic!("`Spanned<T>` takes only a single type argument.")
    };
    if let Some(_) = args.next() {
        panic!("`Spanned<T>` takes only a single type argument.")
    }
    if let GenericArgument::Type(t) = arg {
        t
    } else {
        panic!("`Spanned<T>` takes only a single type argument.")
    }
}

// Return vector of fields that are not `#[space]`
fn process_fields<T>(fields: T) -> Vec<Field>
where
    T: Iterator<Item = Field>,
{
    fields
        .filter(|field| !is_space(field))
        .map(|f| if is_spanned(&f.ty) { map_field(f) } else { f })
        .collect()
}

pub(super) fn struct_ast(s: ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs,
        vis,
        struct_token,
        ident,
        generics,
        fields,
        semi_token,
    } = s;
    let ast_ident = get_ast_ident(&ident);

    let fields = match fields {
        Fields::Named(named) => named.named.into_iter(),
        Fields::Unnamed(unnamed) => unnamed.unnamed.into_iter(),
        Fields::Unit => panic!("Only named fields are supported"),
    };
    let fields = process_fields(fields);

    let fields = if semi_token.is_some() {
        quote! {(#(#fields),*);}
    } else {
        quote! {{#(#fields),*}}
    };
    quote! {
        #(#attrs)*
        #vis #struct_token #ast_ident #generics
        #fields
    }
}

pub(super) fn enum_ast(e: ItemEnum) -> TokenStream {
    let ItemEnum {
        attrs,
        vis,
        enum_token,
        ident,
        generics,
        brace_token: _,
        variants: _,
    } = e;
    let ast_ident = get_ast_ident(&ident);
    quote! {
        #(#attrs)*
        #vis #enum_token #ast_ident #generics {
        }
    }
}
