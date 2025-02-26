use proc_macro2::TokenStream;
use quote2::{quote, ToTokens};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
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
    let ts = ast.to_token_stream();

    let path = Path::new(file);
    let mut result_file_name = path.file_name().unwrap().as_encoded_bytes().to_vec();
    result_file_name[1] = result_file_name[1] + 1;
    let mut file = File::create(
        "crates/example/src/generated/".to_string()
            + std::str::from_utf8(&result_file_name).unwrap(),
    )
    .unwrap();
    file.write_all(ts.to_string().as_bytes()).unwrap();
    proc_macro::TokenStream::new()
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

// #[proc_macro]
// pub fn quote_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input_iter = input.into_iter();
//     for item in input_iter {
//         eprintln!("{}", item.to_string());
//     }
//     proc_macro::TokenStream::new()
// }
