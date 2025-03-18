use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Binary {
    pub left: Rc<Expr>,
    pub right: Rc<Expr>,
    pub operand: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Value {
    pub value: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Binary(Rc<Binary>),
    Value(Rc<Value>),
}
