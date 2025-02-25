use std::rc::Rc;

#[allow(dead_code)]
pub enum Expr {
    Binary(Rc<Expr>, Rc<Expr>, String),
    Value(String),
}
