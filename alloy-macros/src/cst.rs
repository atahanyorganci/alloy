use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, ItemEnum, ItemStruct, Path,
    PathArguments, Type, TypePath, Variant,
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
fn process_struct_fields<T>(fields: T) -> Vec<Field>
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
    let fields = process_struct_fields(fields);

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

fn is_cst(ty: &Type) -> bool {
    if let Type::Path(TypePath { qself: _, path }) = ty {
        if path.leading_colon.is_some() {
            return false;
        }
        let segment = path.segments.last().unwrap();
        segment.ident.to_string().ends_with("CST")
    } else {
        false
    }
}

fn strip_cst_suffix(ident: &Ident) -> Ident {
    let cst = ident.to_string();
    let ast = if let Some(s) = cst.strip_suffix("CST") {
        s.to_string()
    } else {
        panic!("`{}` is not a valid CST identifier", cst)
    };
    Ident::new(&ast, ident.span())
}

fn map_cst(mut ty: Type) -> Type {
    if let Type::Path(TypePath { qself: _, path }) = &mut ty {
        let last = path.segments.last_mut().unwrap();
        last.ident = strip_cst_suffix(&last.ident);
        ty
    } else {
        let ty = quote! {#ty}.to_string();
        panic!("`{ty}` is not a CST type")
    }
}

fn process_enum_fields<T>(fields: T) -> Vec<Field>
where
    T: Iterator<Item = Field>,
{
    // Remove `CST` suffix from each field's type identifier
    fields
        .map(|mut f| {
            if is_cst(&f.ty) {
                f.ty = map_cst(f.ty);
            } else if is_spanned(&f.ty) {
                f.ty = extract_from_spanned(f.ty);
            }
            f
        })
        .collect()
}

fn process_variants<T>(variants: T) -> TokenStream
where
    T: Iterator<Item = Variant>,
{
    let mut stream = TokenStream::new();
    for variant in variants {
        let Variant {
            attrs,
            ident,
            fields,
            discriminant,
        } = variant;
        assert!(discriminant.is_none());

        let fields = match fields {
            Fields::Named(_) => panic!("named fields in enums are not supported"),
            Fields::Unnamed(unnamed) => {
                let fields = process_enum_fields(unnamed.unnamed.into_iter());
                quote! {(#(#fields),*)}
            }
            Fields::Unit => quote! {},
        };

        eprintln!("{} {:?}", ident, fields);
        stream.extend(quote! {
            #(#attrs)*
            #ident #fields,
        });
    }
    stream
}

pub(super) fn enum_ast(e: ItemEnum) -> TokenStream {
    let ItemEnum {
        attrs,
        vis,
        enum_token,
        ident,
        generics,
        brace_token: _,
        variants,
    } = e;
    let ast_ident = get_ast_ident(&ident);
    let variants = process_variants(variants.into_iter());
    quote! {
        #(#attrs)*
        #vis #enum_token #ast_ident #generics {
            #variants
        }
    }
}
