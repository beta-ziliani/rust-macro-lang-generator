use std :: rc :: Rc ; use crate :: l0 :: * ; # [allow (dead_code)] pub trait Visitor { # [allow (unused)] fn enter_binary (self : & mut Self , _target : & Binary) -> bool { true } # [allow (unused)] fn leave_binary (self : & mut Self , _target : & Binary) { } # [allow (unused)] fn enter_value (self : & mut Self , _target : & Value) -> bool { true } # [allow (unused)] fn leave_value (self : & mut Self , _target : & Value) { } } impl crate :: l0 :: Binary { # [allow (unused)] pub fn accept (self : & Rc < Self > , visitor : & mut dyn Visitor) { if visitor . enter_binary (self) { self . left . accept (visitor) ; self . right . accept (visitor) ; visitor . leave_binary (self) } } } impl crate :: l0 :: Value { # [allow (unused)] pub fn accept (self : & Rc < Self > , visitor : & mut dyn Visitor) { if visitor . enter_value (self) { visitor . leave_value (self) } } }