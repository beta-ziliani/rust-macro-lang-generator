mod generated;
mod l0;
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
    // l2 adds an operand type (just the +)
    let _l2_expr = l2::Expr::Binary(
        vec![
            Rc::new(l2::Expr::Value(1.to_string())),
            Rc::new(l2::Expr::Value(2.to_string())),
        ],
        l2::Operand::Plus,
    );
    // l3 replaces the string for values with a literal (i64)
    let _l3_expr = l3::Expr::Binary(
        vec![
            Rc::new(l3::Expr::Value(l3::Value::Literal(1))),
            Rc::new(l3::Expr::Value(l3::Value::Literal(2))),
        ],
        l3::Operand::Plus,
    );

    Ok(())
}
