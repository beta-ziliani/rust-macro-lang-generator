mod generated;
mod l0;
mod l1_stub;
mod l2_stub;
mod l3_stub;
use generated::*;
use std::{error::Error, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let _l0_expr = l0::Expr::Binary(
        Rc::new(l0::Expr::Value("1".to_string())),
        Rc::new(l0::Expr::Value("2".to_string())),
        "+".to_string(),
    );
    // note the change from two arguments to a vec of arguments
    let _l1_expr = l1::Expr::Binary(
        vec![
            Rc::new(l1::Expr::Value(1.to_string())),
            Rc::new(l1::Expr::Value(2.to_string())),
        ],
        "+".to_string(),
    );
    // l2 is just a copy
    let _l1_expr = l2::Expr::Binary(
        vec![
            Rc::new(l2::Expr::Value(1.to_string())),
            Rc::new(l2::Expr::Value(2.to_string())),
        ],
        "+".to_string(),
    );

    Ok(())
}
