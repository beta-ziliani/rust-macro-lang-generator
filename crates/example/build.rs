use core::fmt;
use proc_macro2::TokenStream;
use quote::format_ident;
use std::{error::Error, process::Command};
use syn::{parse2, Item};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

macro_rules! generate_deps {
    () => {
        #[allow(unused_imports)]
        use proc_macro2::TokenStream;
        #[allow(unused_imports)]
        use quote::{quote, ToTokens};
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

        #[allow(dead_code)]
        type Result<T> = std::result::Result<T, Box<dyn Error>>;
    };
}

macro_rules! generate {
    ($gen_func: ident, $from: tt, $to: tt, $func: ident) => {
        pub(crate) fn $gen_func() -> Result<()> {
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

fn generate_visitors(module: &str, items: &[Item]) -> Result<Vec<Item>> {
    let structs: Vec<_> = items
        .iter()
        .filter_map(|i| match i {
            Item::Struct(struct_item) => Some(struct_item),
            _ => None,
        })
        .collect();

    let functions: Vec<_> = structs
        .iter()
        .map(|s| {
            let ident = &s.ident;
            let enter_ident = format_ident!("enter_{}", ident.to_string().to_lowercase());
            let leave_ident = format_ident!("leave_{}", ident.to_string().to_lowercase());
            quote::quote! {
              #[allow(unused)]
              fn #enter_ident (&mut self, _target: &#ident) -> bool {
                true
              }

              #[allow(unused)]
              fn #leave_ident (&mut self, _target: &#ident) {
              }
            }
        })
        .collect();

    let module_name = format_ident!("{}", module);
    let use_ts = quote::quote! {
      use crate::#module_name::*;
    };
    let use_item = parse2(use_ts).unwrap();

    let use_rc_ts = quote::quote! {
      use std::rc::Rc;
    };
    let use_rc_item = parse2(use_rc_ts).unwrap();

    let visitor = quote::quote! {
        #[allow(dead_code)]
        pub trait Visitor {
            #(#functions)*
        }
    };
    let visitor_item: Item = parse2(visitor).unwrap();

    let visitor_ident = format_ident!("{}", "visitor");
    let mut accept_functions: Vec<_> = structs
        .iter()
        .map(|s| {
            let ident = &s.ident;
            let enter_ident = format_ident!("enter_{}", ident.to_string().to_lowercase());
            let leave_ident = format_ident!("leave_{}", ident.to_string().to_lowercase());
            let ident = format_ident!("{}", ident);
            let accept_fields = s.fields.iter().map(|field| {
                let produce = match &field.ty {
                    syn::Type::Path(ty_path) => {
                        // HACK: we should do the opposite and collect those that are in the list of structs
                        ty_path.path.segments.last().unwrap().ident != "String"
                    }
                    _ => true,
                };
                if produce {
                    let field_ident = field.ident.clone().unwrap();
                    quote::quote! {
                      self.#field_ident.accept(#visitor_ident);
                    }
                } else {
                    TokenStream::new()
                }
            });

            quote::quote! {
              impl crate::#module_name::#ident {
                #[allow(unused)]
                pub fn accept (self: &Rc<Self>, visitor: &mut dyn Visitor) {
                  if visitor.#enter_ident(self) {
                    #(#accept_fields)*
                    visitor.#leave_ident(self)
                  }
                }
              }
            }
        })
        .collect();

    let enums: Vec<_> = items
        .iter()
        .filter_map(|i| match i {
            Item::Enum(enum_item) => Some(enum_item),
            _ => None,
        })
        .collect();
    let mut enum_accept_functions: Vec<_> = enums
        .iter()
        .map(|s| {
            let ident = &s.ident;
            let ident = format_ident!("{}", ident);
            let accept_variants = s.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                quote::quote! {
                  Self::#variant_ident(ref x) => x.accept(#visitor_ident),
                }
            });

            quote::quote! {
              impl crate::#module_name::#ident {
                #[allow(unused)]
                pub fn accept (&self, visitor: &mut dyn Visitor) {
                  match self {
                    #(#accept_variants)*
                  }
                }
              }
            }
        })
        .collect();

    accept_functions.append(&mut enum_accept_functions);

    let mut accept_function_items: Vec<_> = accept_functions
        .iter()
        .map(|ts| parse2(ts.clone()).unwrap())
        .collect();
    let mut result = vec![use_rc_item, use_item, visitor_item];
    result.append(&mut accept_function_items);
    Ok(result)
}

macro_rules! visitors {
    ($gen_func: ident, $from_mod: tt, $from: tt, $to: tt) => {
        pub(crate) fn $gen_func() -> Result<()> {
            let content = fs::read_to_string($from)?;
            let mut ast = syn::parse_file(&content)?;
            ast.items = crate::generate_visitors($from_mod, &ast.items)?;
            let ts = ast.to_token_stream();

            let mut file = File::create($to)?;
            file.write_all(ts.to_string().as_bytes())?;
            Ok(())
        }
    };
}

mod transform {

    generate_deps!();
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
    use syn::Item::Struct;

    use crate::DerivationError;

    generate_deps!();

    pub fn find_struct(items: &mut [Item], name: &str) -> Result<usize> {
        let ix = items
            .iter()
            .position(|i| {
                matches!(i, Struct(_))
                    && if let Struct(item) = i {
                        item.ident == name
                    } else {
                        false
                    }
            })
            .ok_or::<Box<dyn Error>>(DerivationError::new("no struct").into())?;
        Ok(ix)
    }

    fn generate_l1(items: &mut [Item]) -> Result<()> {
        let item_ix = find_struct(items, "Binary")?;
        let ts = quote! {
            #[allow(dead_code)]
            #[derive(Debug, PartialEq, Eq)]
            pub struct Binary {
                pub exprs: Vec<Rc<Expr>>,
                pub operand: String,
            }
        };

        items[item_ix] = parse2(ts).unwrap();
        Ok(())
    }

    generate!(l1, "./src/l0.rs", "./src/generated/l1.rs", generate_l1);

    visitors!(
        visitors_l0,
        "l0",
        "./src/l0.rs",
        "./src/generated/l0_visitors.rs"
    );

    // visitors!(
    //     visitors_l1,
    //     "l1",
    //     "./src/generated/l1.rs",
    //     "./src/generated/l1_visitors.rs"
    // );
}

mod resolve_operands {
    generate_deps!();

    fn generate_l2(items: &mut Vec<Item>) -> Result<()> {
        insert_operand_enum(items);

        let index = crate::linearization::find_struct(items, "Binary").unwrap();
        items[index] = parse2(quote! {
            #[allow(dead_code)]
            #[derive(Debug, PartialEq, Eq)]
            pub struct Binary {
                pub exprs: Vec<Rc<Expr>>,
                pub operand: Operand,
            }
        })
        .unwrap();
        Ok(())
    }

    fn insert_operand_enum(items: &mut Vec<Item>) {
        let operand_enum = parse2(quote! {
            #[derive(Debug, PartialEq, Eq)]
            pub enum Operand {
                Plus,
            }
        })
        .unwrap();

        items.push(operand_enum)
    }

    generate!(
        l2,
        "./src/generated/l1.rs",
        "./src/generated/l2.rs",
        generate_l2
    );

    // visitors!(
    //     visitors,
    //     "l2",
    //     "./src/generated/l2.rs",
    //     "./src/generated/l2_visitors.rs"
    // );
}

fn main() -> Result<()> {
    println!("cargo::rustc-env=RUST_BACKTRACE=1");
    linearization::l1()?;
    linearization::visitors_l0()?;

    // TODO: Visitors do not take into account Vec<_> yet
    // linearization::visitors_l1()?;

    resolve_operands::l2()?;

    // TODO: ditto
    // resolve_operands::visitors()?;

    let mut command = Command::new("cargo-fmt");
    println!("cargo::warning={:?}", command.output());
    Ok(())
}
