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
    if let Enum(item) = &mut items[enum_ix] {
        transform_enum(item)
    } else {
        unreachable!()
    }
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

fn transform_enum(item: &mut ItemEnum) {
    if item.ident == "Expr" {
        transform_variants(&mut item.variants)
    }
}

fn transform_variants(variants: &mut Punctuated<Variant, Comma>) {
    if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Binary") {
        let mut new_variant = TokenStream::new();
        quote!(new_variant, {
          Binary(Vec<Rc<Expr>>, String)
        });
        variants[variant_ix] = parse2(new_variant).unwrap()
    }
}

// #[proc_macro]
// pub fn quote_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input_iter = input.into_iter();
//     for item in input_iter {
//         eprintln!("{}", item.to_string());
//     }
//     proc_macro::TokenStream::new()
// }
