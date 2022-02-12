use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, AngleBracketedGenericArguments, Field, Fields, FieldsNamed,
    FieldsUnnamed, GenericArgument, Generics, Index, ItemEnum, ItemStruct, Path, PathArguments,
    Type, TypePath, Variant,
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

fn is_boxed(ty: &Type) -> bool {
    if let Type::Path(TypePath { qself, path }) = ty {
        if qself.is_some() {
            return false;
        }
        compare_path(path, vec!["std", "boxed", "Box"])
    } else {
        false
    }
}

fn replace_type(ty: &mut Type, new_ty: Type) {
    let segment = if let Type::Path(tp) = ty {
        tp.path.segments.last_mut().unwrap()
    } else {
        panic!("only `TypePath`'s generic arguments can be replaced.")
    };
    if let PathArguments::AngleBracketed(a) = &mut segment.arguments {
        let mut args = Punctuated::new();
        args.push(GenericArgument::Type(new_ty));
        a.args = args;
    }
}

fn map_field(mut field: Field) -> (FieldType, Field) {
    // Check if field has `#[space]` if so return `FieldType::Space` and field
    if is_space(&field) {
        return (FieldType::Space, field);
    }

    let mut field_type = FieldType::Simple;

    // Check if field is `Spanned<T>`
    if is_spanned(&field.ty) {
        field.ty = match try_extract_generic(field.ty) {
            Ok(ty) => ty,
            Err(_) => {
                panic!("`Spanned<T>` type must be generic with single arg");
            }
        };
        field_type = FieldType::Spanned;
    }

    // Check if field is `Box<T>` if not it can't be CST since CSTs are self-referential
    // and require `Box` or other reference types.
    if !is_boxed(&field.ty) {
        return (field_type, field);
    }

    // Extract generic argument from `Box<T>`
    let boxed = match try_extract_generic(field.ty.clone()) {
        Ok(ty) => ty,
        Err(_) => {
            panic!("`Box<T>` type must be generic with single arg");
        }
    };

    // if boxed type isn't CST don't replace it
    if !is_cst(&boxed) {
        return (field_type, field);
    }

    let mut ast = map_cst(boxed);
    remove_generics(&mut ast);
    replace_type(&mut field.ty, ast);

    match field_type {
        FieldType::Simple => (FieldType::CST, field),
        FieldType::Spanned => (FieldType::SpannedCST, field),
        FieldType::Space | FieldType::CST | FieldType::SpannedCST => unreachable!(),
    }
}

fn try_extract_generic(ty: Type) -> Result<Type, ()> {
    let segment = if let Type::Path(TypePath { qself: _, path }) = ty {
        path.segments.into_iter().last().unwrap()
    } else {
        unreachable!()
    };
    if let PathArguments::AngleBracketed(args) = segment.arguments {
        try_extract_single_generic_arg(args)
    } else {
        Err(())
    }
}

fn try_extract_single_generic_arg(args: AngleBracketedGenericArguments) -> Result<Type, ()> {
    let mut args = args.args.into_iter();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        return Err(());
    };
    if let Some(_) = args.next() {
        return Err(());
    }
    if let GenericArgument::Type(t) = arg {
        Ok(t)
    } else {
        return Err(());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FieldType {
    CST,
    Space,
    Simple,
    Spanned,
    SpannedCST,
}

fn remove_generics(ty: &mut Type) {
    if let Type::Path(TypePath { qself, path }) = ty {
        if qself.is_some() {
            return;
        }
        let last = path.segments.last_mut().unwrap();
        last.arguments = PathArguments::None;
    }
}

fn impl_block(from: &Ident, into: &Ident, generics: Generics, body: TokenStream) -> TokenStream {
    quote! {
        impl #generics From<#from #generics> for #into {
            fn from(cst: #from #generics) -> Self {
                #body
            }
        }
    }
}

fn impl_named_struct(
    from: &Ident,
    into: &Ident,
    generics: Generics,
    fields: StructFields,
) -> TokenStream {
    let mut assign_vars = TokenStream::new();
    let mut assign_fields = TokenStream::new();
    for (field_type, field) in fields.into_iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_assignment = match field_type {
            FieldType::CST => {
                let ty = try_extract_generic(field.ty.clone()).unwrap();
                assign_vars.extend(quote! {
                    let #ident: #ty = (*cst.#ident).into();
                });
                quote! {
                    #ident: std::boxed::Box::from(#ident),
                }
            }
            FieldType::Space => continue,
            FieldType::Simple => quote! {
                #ident: cst.#ident,
            },
            FieldType::Spanned => quote! {
                #ident: cst.#ident.ast.into(),
            },
            FieldType::SpannedCST => {
                let ty = try_extract_generic(field.ty.clone()).unwrap();
                assign_vars.extend(quote! {
                    let #ident: #ty = (*cst.#ident.ast).into();
                });
                quote! {
                    #ident: std::boxed::Box::from(#ident),
                }
            }
        };
        assign_fields.extend(field_assignment);
    }
    let body = quote! {
        #assign_vars
        #into {
            #assign_fields
        }
    };
    impl_block(from, into, generics, body)
}

fn impl_tuple_struct(
    from: &Ident,
    into: &Ident,
    generics: Generics,
    fields: StructFields,
) -> TokenStream {
    let mut assign_vars = TokenStream::new();
    let mut assign_fields = Vec::new();
    for (i, (field_type, field)) in fields.into_iter().enumerate() {
        let idx = Index::from(i);
        match field_type {
            FieldType::CST => {
                let ident = Ident::new(&format!("var{i}"), Span::call_site());
                let ty = try_extract_generic(field.ty.clone()).unwrap();
                assign_vars.extend(quote! {
                    let ident: #ty = (*cst.#idx).into();
                });
                assign_fields.push(quote! {
                    #ident
                });
            }
            FieldType::Space => continue,
            FieldType::Simple => {
                assign_fields.push(quote! {
                    cst.#idx
                });
            }
            FieldType::Spanned => {
                assign_fields.push(quote! {
                    cst.#idx.ast
                });
            }
            FieldType::SpannedCST => {
                let ident = Ident::new(&format!("var{i}"), Span::call_site());

                let ty = try_extract_generic(field.ty.clone()).unwrap();
                assign_vars.extend(quote! {
                    let ident: #ty = (*cst.#idx.ast).into();
                });
                assign_fields.push(quote! {
                    #ident
                });
            }
        };
    }

    let body = quote! {
        #assign_vars
        #into(#(#assign_fields),*)
    };
    impl_block(from, into, generics, body)
}

fn impl_enum<T>(from: &Ident, into: &Ident, generics: Generics, variants: T) -> TokenStream
where
    T: Iterator<Item = Variant>,
{
    let variants = variants
        .map(|v| {
            let ident = v.ident;
            let field = extract_enum_field(v.fields);
            if is_spanned(&field.ty) {
                quote! {
                    #from::#ident(cst) => {
                        Self::#ident(cst.ast)
                    }
                }
            } else {
                quote! {
                    #from::#ident(cst) => {
                        Self::#ident(cst.into())
                    }
                }
            }
        })
        .collect::<Vec<_>>();
    let body = quote! {
        match cst {
            #(#variants)*
        }
    };
    impl_block(from, into, generics, body)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum StructType {
    Named,
    Tuple,
}

struct StructFields {
    pub fields: Vec<(FieldType, Field)>,
    pub ty: StructType,
}

impl IntoIterator for StructFields {
    type Item = (FieldType, Field);
    type IntoIter = std::vec::IntoIter<(FieldType, Field)>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl From<FieldsNamed> for StructFields {
    fn from(fields: FieldsNamed) -> Self {
        let fields = fields.named.into_iter().map(map_field).collect::<Vec<_>>();
        let ty = StructType::Named;
        Self { fields, ty }
    }
}

impl From<FieldsUnnamed> for StructFields {
    fn from(fields: FieldsUnnamed) -> Self {
        let fields = fields
            .unnamed
            .into_iter()
            .map(map_field)
            .collect::<Vec<_>>();
        let ty = StructType::Tuple;
        Self { fields, ty }
    }
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

    let struct_fields: StructFields = match fields {
        Fields::Named(named) => named.into(),
        Fields::Unnamed(unnamed) => unnamed.into(),
        Fields::Unit => panic!("Only named fields are supported"),
    };

    let fields = struct_fields
        .fields
        .iter()
        .filter(|(ty, _)| *ty != FieldType::Space)
        .cloned()
        .map(|(_ty, field)| field)
        .collect::<Vec<_>>();

    let trait_impl = match struct_fields.ty {
        StructType::Named => impl_named_struct(&ident, &ast_ident, generics, struct_fields),
        StructType::Tuple => impl_tuple_struct(&ident, &ast_ident, generics, struct_fields),
    };

    let fields = if semi_token.is_some() {
        quote! {(#(#fields),*);}
    } else {
        quote! {{#(#fields),*}}
    };
    quote! {
        #(#attrs)*
        #vis #struct_token #ast_ident
        #fields

        #trait_impl
    }
}

fn is_cst(ty: &Type) -> bool {
    if let Type::Path(TypePath { qself: _, path }) = ty {
        if path.leading_colon.is_some() {
            false
        } else {
            let segment = path.segments.last().unwrap();
            segment.ident.to_string().ends_with("CST")
        }
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

fn process_enum_field(mut field: Field) -> Field {
    // Remove `CST` suffix from each field's type identifier
    if is_cst(&field.ty) {
        field.ty = map_cst(field.ty);
    } else if is_spanned(&field.ty) {
        field.ty = if let Ok(ty) = try_extract_generic(field.ty) {
            ty
        } else {
            panic!("`Spanned<T>` takes only a single type argument.")
        }
    }
    remove_generics(&mut field.ty);
    field
}

fn extract_enum_field(fields: Fields) -> Field {
    // enums variant's fields are always a single unnamed field
    if let Fields::Unnamed(unnamed) = fields {
        let mut iter = unnamed.unnamed.into_iter();
        let field = iter.next().unwrap();
        if iter.next().is_some() {
            panic!("Only one unnamed field is supported in an enum variant")
        }
        field
    } else {
        panic!("Enum CSTs can only have a single unnamed field")
    }
}

fn map_enum_variant_field(fields: Fields) -> Field {
    process_enum_field(extract_enum_field(fields))
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

        let field = map_enum_variant_field(fields);
        stream.extend(quote! {
            #(#attrs)*
            #ident(#field),
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
    let trait_impl = impl_enum(&ident, &ast_ident, generics, variants.iter().cloned());
    let variants = process_variants(variants.into_iter());
    quote! {
        #(#attrs)*
        #vis #enum_token #ast_ident {
            #variants
        }

        #trait_impl
    }
}
