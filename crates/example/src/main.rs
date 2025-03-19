mod generated;
mod l0;
use generated::*;
use std::{error::Error, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let l0_expr = l0::Expr::Binary(Rc::new(l0::Binary {
        left: Rc::new(l0::Expr::Value(Rc::new(l0::Value {
            value: "1".to_string(),
        }))),
        right: Rc::new(l0::Expr::Value(Rc::new(l0::Value {
            value: "2".to_string(),
        }))),
        operand: "+".to_string(),
    }));
    // note the change from two arguments to a vec of arguments
    let l1_expr = l1::Expr::Binary(Rc::new(l1::Binary {
        exprs: vec![
            Rc::new(l1::Expr::Value(Rc::new(l1::Value {
                value: 1.to_string(),
            }))),
            Rc::new(l1::Expr::Value(Rc::new(l1::Value {
                value: 2.to_string(),
            }))),
        ],
        operand: "+".to_string(),
    }));

    assert_eq!(l1_expr, *l0_expr_to_l1(&l0_expr));

    let mut visitor = L0toL1::new();
    l0_expr.accept(&mut visitor);
    assert_eq!(&l1_expr, visitor.value());

    //l2 adds an operand type (just the +)
    let _l2_expr = l2::Expr::Binary(Rc::new(l2::Binary {
        exprs: vec![
            Rc::new(l2::Expr::Value(Rc::new(l2::Value {
                value: 1.to_string(),
            }))),
            Rc::new(l2::Expr::Value(Rc::new(l2::Value {
                value: 2.to_string(),
            }))),
        ],
        operand: l2::Operand::Plus,
    }));

    Ok(())
}

fn l0_expr_to_l1(l0_expr: &l0::Expr) -> Rc<l1::Expr> {
    let binary = |left, right, operand: &String| {
        Rc::new(l1::Expr::Binary(
            l1::Binary {
                exprs: vec![left, right],
                operand: operand.clone(),
            }
            .into(),
        ))
    };
    let value = |str: &String| Rc::new(l1::Expr::Value(l1::Value { value: str.clone() }.into()));
    l0_expr_map(l0_expr, &binary, &value)
}

fn l0_expr_map<B, V, T>(l0_expr: &l0::Expr, binary_f: &B, value_f: &V) -> T
where
    B: Fn(T, T, &String) -> T,
    V: Fn(&String) -> T,
{
    match l0_expr {
        l0::Expr::Binary(binary) => {
            let left = l0_expr_map(&binary.left, binary_f, value_f);
            let right = l0_expr_map(&binary.right, binary_f, value_f);
            binary_f(left, right, &binary.operand)
        }
        l0::Expr::Value(value) => value_f(&value.value),
    }
}

struct L0toL1 {
    stack: Vec<Rc<l1::Expr>>,
}

impl L0toL1 {
    pub fn new() -> L0toL1 {
        L0toL1 { stack: vec![] }
    }

    pub fn value(&self) -> &l1::Expr {
        self.stack.first().unwrap()
    }
}

impl l0_visitors::Visitor for L0toL1 {
    fn leave_value(&mut self, target: &l0::Value) {
        self.stack.push(Rc::new(l1::Expr::Value(
            l1::Value {
                value: target.value.clone(),
            }
            .into(),
        )));
    }

    fn leave_binary(&mut self, target: &l0::Binary) {
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();
        let binary = l1::Expr::Binary(
            l1::Binary {
                exprs: vec![Rc::clone(&left), Rc::clone(&right)],
                operand: target.operand.clone(),
            }
            .into(),
        );
        self.stack.push(Rc::new(binary));
    }
}
