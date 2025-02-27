use core::fmt;
use std::error::Error;

macro_rules! generate_deps {
    () => {
        #[allow(unused_imports)]
        use proc_macro2::TokenStream;
        #[allow(unused_imports)]
        use quote2::{quote, ToTokens};
        #[allow(unused_imports)]
        use std::error::Error;
        #[allow(unused_imports)]
        use std::fs::{self, File};
        #[allow(unused_imports)]
        use std::io::Write;
        #[allow(unused_imports)]
        use syn::punctuated::Punctuated;
        #[allow(unused_imports)]
        use syn::token::Comma;
        #[allow(unused_imports)]
        use syn::Item;
        #[allow(unused_imports)]
        use syn::Item::Enum;
        #[allow(unused_imports)]
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

#[derive(Debug)]
struct DerivationError {
    problem: String,
}

impl fmt::Display for DerivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.problem)
    }
}
impl Error for DerivationError {}
impl DerivationError {
    pub fn new(problem: &str) -> Self {
        DerivationError {
            problem: problem.to_string(),
        }
    }
}
mod linearization {
    use crate::DerivationError;

    generate_deps!();

    pub fn find_expr(items: &Vec<Item>) -> Result<usize, Box<dyn Error>> {
        items
            .iter()
            .position(|i| {
                matches!(i, Enum(_))
                    && if let Enum(item) = i {
                        item.ident == "Expr"
                    } else {
                        unreachable!()
                    }
            })
            .ok_or(DerivationError::new("no Expr enum").into())
    }

    fn generate_l1(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let enum_ix = find_expr(items)?;
        if let Enum(item) = &mut items[enum_ix] {
            transform_enum(item)
        } else {
            unreachable!()
        }
    }

    fn transform_enum(item: &mut ItemEnum) -> Result<(), Box<dyn std::error::Error>> {
        transform_variants(&mut item.variants)
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

mod resolve_operands {
    use crate::DerivationError;

    generate_deps!();

    fn generate_l2(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        add_enum(items)?;

        let enum_ix = crate::linearization::find_expr(items)?;
        if let Enum(item) = &mut items[enum_ix] {
            transform_enum(item)
        } else {
            unreachable!()
        }
    }

    fn add_enum(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut new_enum = TokenStream::new();
        quote!(new_enum, {
            pub enum Operand {
                Plus,
            }
        });
        let operand_enum = parse2(new_enum)?;
        items.push(operand_enum);
        Ok(())
    }

    fn transform_enum(item: &mut ItemEnum) -> Result<(), Box<dyn std::error::Error>> {
        transform_variants(&mut item.variants)
    }

    fn transform_variants(
        variants: &mut Punctuated<Variant, Comma>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Binary") {
            let mut new_variant = TokenStream::new();
            quote!(new_variant, {
              Binary(Vec<Rc<Expr>>, Operand)
            });
            variants[variant_ix] = parse2(new_variant)?;
            Ok(())
        } else {
            Err(DerivationError::new("no Binary variant").into())
        }
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
    resolve_operands::l2()?;
    l3_stub::l3()?;
    Ok(())
}
