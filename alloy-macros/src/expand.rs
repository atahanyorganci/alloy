use proc_macro2::TokenStream;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, ExprLit, ExprParen, ExprUnary, Lit};

fn expand_binary(expr: &ExprBinary) -> TokenStream {
    let op = match expr.op {
        BinOp::Add(_) => quote! { Add },
        BinOp::Sub(_) => quote! { Subtract },
        BinOp::Mul(_) => quote! { Multiply },
        BinOp::Div(_) => quote! { Divide },
        BinOp::Rem(_) => quote! { Reminder },
        BinOp::And(_) => quote! { LogicalAnd },
        BinOp::Or(_) => quote! { LogicalOr },
        BinOp::BitXor(_) => quote! { LogicalXor },
        BinOp::Eq(_) => quote! { Equal },
        BinOp::Lt(_) => quote! { LessThan },
        BinOp::Le(_) => quote! { LessThanEqual },
        BinOp::Ne(_) => quote! { NotEqual },
        BinOp::Ge(_) => quote! { GreaterThan },
        BinOp::Gt(_) => quote! { GreaterThanEqual },
        _ => panic!("Unsupported binary expression"),
    };
    let left = expand_expr(&expr.left);
    let right = expand_expr(&expr.right);
    let binary = quote! {
        alloy::ast::expression::binary::BinaryExpression {
            left: std::boxed::Box::from(#left),
            right: std::boxed::Box::from(#right),
            operator: alloy::ast::expression::binary::BinaryOperator::#op
        }
    };
    quote! {alloy::ast::expression::Expression::Binary(#binary)}
}

fn expand_lit(expr: &ExprLit) -> TokenStream {
    let value = match &expr.lit {
        Lit::Str(_) => unimplemented!(),
        Lit::ByteStr(_) => unimplemented!(),
        Lit::Byte(_) => unimplemented!(),
        Lit::Char(_) => unimplemented!(),
        Lit::Int(int) => {
            let value: i64 = int.base10_parse().unwrap();
            quote! {
                alloy::ast::value::Value::Integer(#value)
            }
        }
        Lit::Float(float) => {
            let value: f64 = float.base10_parse().unwrap();
            quote! {
                alloy::ast::value::Value::Float(#value)
            }
        }
        Lit::Bool(bool) => {
            if bool.value {
                quote!(alloy::ast::value::Value::True)
            } else {
                quote!(alloy::ast::value::Value::False)
            }
        }
        Lit::Verbatim(_) => todo!(),
    };
    quote! {
        alloy::ast::expression::Expression::Value(#value)
    }
}

fn expand_paren(expr: &ExprParen) -> TokenStream {
    expand_expr(&expr.expr)
}

fn expand_unary(expr: &ExprUnary) -> TokenStream {
    let operand = expand_expr(&expr.expr);
    let op = match expr.op {
        syn::UnOp::Deref(_) => panic!("Unsupported unary expression"),
        syn::UnOp::Not(_) => quote! { Not },
        syn::UnOp::Neg(_) => quote! { Minus },
    };
    let unary = quote! {
        alloy::ast::expression::unary::UnaryExpression {
            operator: alloy::ast::expression::unary::UnaryOperator::#op,
            expression: std::boxed::Box::from(#operand)
        }
    };
    quote! {
        alloy::ast::expression::Expression::Unary(#unary)
    }
}

pub(crate) fn expand_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Binary(binary) => expand_binary(binary),
        Expr::Lit(lit) => expand_lit(lit),
        Expr::Paren(paren) => expand_paren(paren),
        Expr::Unary(unary) => expand_unary(unary),
        _ => panic!("Unsupported expression type"),
    }
}
