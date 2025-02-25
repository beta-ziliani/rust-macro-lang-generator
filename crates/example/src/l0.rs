mod l0 {
    use std::rc::Rc;

    pub enum Expr {
        Binary(Rc<Expr>, Rc<Expr>, String),
        Value(String),
    }
}
