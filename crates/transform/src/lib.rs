use proc_macro2::TokenStream;
use quote2::{quote, ToTokens};
use std::fs;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Item::Enum;
use syn::{parse2, ItemEnum, Variant};

extern crate proc_macro;

#[proc_macro]
pub fn run(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as syn::LitStr);
    let file = args.token().to_string();
    let file = file.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap();
    let content = fs::read_to_string(file).unwrap();
    let mut ast = syn::parse_file(&content).unwrap();

    let items = &mut ast.items;
    let enum_ix = items.iter().position(|i| matches!(i, Enum(_))).unwrap();

    let result = match &mut items[enum_ix] {
        Enum(item) => Enum(transform_enum(item)),
        _ => panic!("Not an enum"),
    };

    items[enum_ix] = result;
    ast.to_token_stream().into()
}

fn transform_enum(item: &ItemEnum) -> ItemEnum {
    let mut result = item.clone();
    if item.ident == "Expr" {
        result.variants = transform_variants(&item.variants)
    }
    result
}

fn transform_variants(variants: &Punctuated<Variant, Comma>) -> Punctuated<Variant, Comma> {
    let mut result = variants.clone();
    match variants.first() {
        Some(variant) => {
            if variant.ident == "Binary" {
                let mut new_variant = TokenStream::new();
                quote!(new_variant, {
                  Binary(Vec<Rc<Expr>>, String)
                });
                result[0] = parse2(new_variant).unwrap()
            }
        }
        None => (),
    }
    result
}
