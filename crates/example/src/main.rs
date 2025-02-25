mod l0;
mod l1;
use std::{error::Error, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let _l0_expr = l0::Expr::Binary(
        Rc::new(l0::Expr::Value("1".to_string())),
        Rc::new(l0::Expr::Value("2".to_string())),
        "+".to_string(),
    );
    let _en = l1::Expr::Binary(
        vec![
            Rc::new(l1::Expr::Value(1.to_string())),
            Rc::new(l1::Expr::Value(2.to_string())),
        ],
        "+".to_string(),
    );
    Ok(())
}
