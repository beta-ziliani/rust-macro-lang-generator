macro_rules! generate_deps {
    () => {
        use proc_macro2::TokenStream;
        use quote2::{quote, ToTokens};
        use std::error::Error;
        use std::fs::{self, File};
        use std::io::Write;
        use syn::punctuated::Punctuated;
        use syn::token::Comma;
        use syn::Item;
        use syn::Item::Enum;
        use syn::{parse2, ItemEnum, Variant};
    };
}

macro_rules! generate {
    ($gen_func: ident, $from: tt, $to: tt, $func: ident) => {
        pub(crate) fn $gen_func() -> Result<(), Box<dyn Error>> {
            let content = fs::read_to_string($from)?;
            let mut ast = syn::parse_file(&content)?;
            let items = &mut ast.items;
            $func(items)?;
            let ts = ast.to_token_stream();

            let mut file = File::create($to)?;
            file.write_all(ts.to_string().as_bytes())?;
            Ok(())
        }
    };
}

mod linearization {
    use std::fmt;

    generate_deps!();

    #[derive(Debug)]
    struct NoEnumFound;

    impl fmt::Display for NoEnumFound {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "No enum found")
        }
    }
    impl Error for NoEnumFound {}

    fn generate_l1(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let enum_ix = items
            .iter()
            .position(|i| matches!(i, Enum(_)))
            .ok_or(NoEnumFound)?;
        if let Enum(item) = &mut items[enum_ix] {
            transform_enum(item)
        } else {
            unreachable!()
        }
    }

    fn transform_enum(item: &mut ItemEnum) -> Result<(), Box<dyn std::error::Error>> {
        if item.ident == "Expr" {
            transform_variants(&mut item.variants)
        } else {
            Ok(())
        }
    }

    fn transform_variants(
        variants: &mut Punctuated<Variant, Comma>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Binary") {
            let mut new_variant = TokenStream::new();
            quote!(new_variant, {
              Binary(Vec<Rc<Expr>>, String)
            });
            variants[variant_ix] = parse2(new_variant)?
        }
        Ok(())
    }

    generate!(l1, "./src/l0.rs", "./src/generated/l1.rs", generate_l1);
}

mod l2_stub {
    generate_deps!();

    fn generate_l2(_items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    generate!(
        l2,
        "./src/generated/l1.rs",
        "./src/generated/l2.rs",
        generate_l2
    );
}

mod l3_stub {
    generate_deps!();

    fn generate_l3(_items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    generate!(
        l3,
        "./src/generated/l2.rs",
        "./src/generated/l3.rs",
        generate_l3
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    linearization::l1()?;
    l2_stub::l2()?;
    l3_stub::l3()?;
    Ok(())
}
