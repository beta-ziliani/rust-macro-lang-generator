mod l0;
use std::{error::Error, rc::Rc};
use transform::run;

run!();

fn main() -> Result<(), Box<dyn Error>> {
    let en = l1::Expr::Binary(
        vec![
            Rc::new(l1::Expr::Value(1.to_string())),
            Rc::new(l1::Expr::Value(2.to_string())),
        ],
        "+".to_string(),
    );
    Ok(())
}
