use proc_macro2::TokenStream;
use quote2::{quote, ToTokens};
use std::error::Error;
use std::fs;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Item::{Enum, Mod};
use syn::{parse2, Ident, ItemEnum, Variant};

extern crate proc_macro;

#[proc_macro]
pub fn run(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let content =
        fs::read_to_string("/Users/beta/projects/nomic/tests/rust-ldw/crates/example/src/l0.rs")
            .unwrap();
    let mut ast = syn::parse_file(&content).unwrap();
    let module_ix = ast.items.iter().position(|i| matches!(i, Mod(_))).unwrap();
    let module = match &mut ast.items[module_ix] {
        Mod(item) => item,
        _ => panic!("Not a module"),
    };

    {
        let items = match &mut module.content {
            Some((_, items)) => items,
            _ => panic!("No content"),
        };
        let enum_ix = items.iter().position(|i| matches!(i, Enum(_))).unwrap();

        let result = match &mut items[enum_ix] {
            Enum(item) => Enum(transform_enum(item)),
            _ => panic!("Not an enum"),
        };

        items[enum_ix] = result;
    }
    module.ident = Ident::new("l1", module.span());
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
