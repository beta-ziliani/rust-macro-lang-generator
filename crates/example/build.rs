macro_rules! generate_deps {
    () => {
        use proc_macro2::TokenStream;
        use quote2::{quote, ToTokens};
        use std::fs::{self, File};
        use std::io::Write;
        use syn::punctuated::Punctuated;
        use syn::token::Comma;
        use syn::Item::Enum;
        use syn::{parse2, ItemEnum, Variant};
    };
}

macro_rules! generate {
  ($func: ident, $from: tt, $to: tt) => {

      pub(crate) fn $func() {
          let content = fs::read_to_string($from).unwrap();
          let mut ast = syn::parse_file(&content).unwrap();
          let items = &mut ast.items;
          let enum_ix = items.iter().position(|i| matches!(i, Enum(_))).unwrap();
          if let Enum(item) = &mut items[enum_ix] {
              transform_enum(item)
          } else {
              unreachable!()
          }
          let ts = ast.to_token_stream();

          let mut file = File::create($to).unwrap();
          file.write_all(ts.to_string().as_bytes()).unwrap();
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
  };
}

mod l1_stub {
    generate_deps!();
    generate!(l1, "./src/l0.rs", "./src/generated/l1.rs");
}

mod l2_stub {
    generate_deps!();
    generate!(l2, "./src/generated/l1.rs", "./src/generated/l2.rs");
}

mod l3_stub {
    generate_deps!();
    generate!(l3, "./src/generated/l2.rs", "./src/generated/l3.rs");
}

fn main() -> Result<(), String> {
    l1_stub::l1();
    l2_stub::l2();
    l3_stub::l3();
    Ok(())
}
