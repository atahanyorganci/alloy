use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Expr, LitStr};

mod expand;

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Expr);
    expand::expand_expr(&ast).into()
}

#[proc_macro]
pub fn assert_expr(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let ast = parse_macro_input!(input as Expr);
    let lhs = expand::expand_expr(&ast);
    let litstr = LitStr::new(&string, lhs.span());
    let gen = quote! {
        assert_eq!(
            #lhs,
            alloy::parser::parse_rule::<alloy::ast::expression::Expression>(
                alloy::parser::Rule::expression,
                #litstr
            ).unwrap()
        );
    };
    gen.into()
}
