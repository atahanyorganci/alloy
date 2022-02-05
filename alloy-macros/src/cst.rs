use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Fields, ItemEnum, ItemStruct};

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

// Return vector of fields that are not `#[space]`
fn process_fields<T>(fields: T) -> Vec<Field>
where
    T: Iterator<Item = Field>,
{
    fields.filter(|field| !is_space(field)).collect()
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
        eprintln!("{fields:#?}");
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
