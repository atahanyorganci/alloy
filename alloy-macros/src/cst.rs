use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ItemEnum, ItemStruct};

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

pub(super) fn struct_ast(s: ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs,
        vis,
        struct_token,
        ident,
        generics,
        fields: _,
        semi_token,
    } = s;
    let ast_ident = get_ast_ident(&ident);
    let fields = if semi_token.is_some() {
        quote! {();}
    } else {
        quote! {{}}
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
