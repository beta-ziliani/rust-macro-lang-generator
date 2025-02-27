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

macro_rules! wrapped_quote {
    ($blk: tt) => {{
        let mut new_variant = TokenStream::new();
        quote!(new_variant, $blk);
        parse2(new_variant)?
    }};
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

    pub fn find_enum<'a>(
        items: &'a mut Vec<Item>,
        name: &str,
    ) -> Result<&'a mut ItemEnum, Box<dyn Error>> {
        let enum_ix = items
            .iter()
            .position(|i| {
                matches!(i, Enum(_))
                    && if let Enum(item) = i {
                        item.ident == name
                    } else {
                        unreachable!()
                    }
            })
            .ok_or::<Box<dyn Error>>(DerivationError::new("no Expr enum").into())?;
        if let Enum(item) = &mut items[enum_ix] {
            Ok(item)
        } else {
            unreachable!()
        }
    }

    fn generate_l1(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let enum_item = find_enum(items, "Expr")?;
        transform_variant(&mut enum_item.variants)
    }

    fn transform_variant(
        variants: &mut Punctuated<Variant, Comma>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Binary") {
            variants[variant_ix] = wrapped_quote!({
              Binary(Vec<Rc<Expr>>, String)
            })
        }
        Ok(())
    }

    generate!(l1, "./src/l0.rs", "./src/generated/l1.rs", generate_l1);
}

mod resolve_operands {
    use crate::DerivationError;

    generate_deps!();

    fn generate_l2(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        insert_operand_enum(items)?;

        let enum_item = crate::linearization::find_enum(items, "Expr")?;
        transform_variant(&mut enum_item.variants)
    }

    fn insert_operand_enum(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
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

    fn transform_variant(
        variants: &mut Punctuated<Variant, Comma>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Binary") {
            variants[variant_ix] = wrapped_quote!({
              Binary(Vec<Rc<Expr>>, Operand)
            });
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

mod resolve_values {
    use crate::DerivationError;

    generate_deps!();

    fn generate_l3(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        insert_value_enum(items)?;
        let enum_item = crate::linearization::find_enum(items, "Expr")?;
        transform_variant(&mut enum_item.variants)
    }

    fn insert_value_enum(items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut new_enum = TokenStream::new();
        quote!(new_enum, {
            #[allow(dead_code)]
            pub enum Value {
                Literal(i64),
            }
        });
        let value_enum = parse2(new_enum)?;
        items.push(value_enum);
        Ok(())
    }

    fn transform_variant(
        variants: &mut Punctuated<Variant, Comma>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(variant_ix) = variants.iter().position(|v| v.ident == "Value") {
            variants[variant_ix] = wrapped_quote!({ Value(Value) });
            Ok(())
        } else {
            Err(DerivationError::new("no Value variant").into())
        }
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
    resolve_values::l3()?;
    Ok(())
}
