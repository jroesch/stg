use ast::*;

pub fn var(s: &'static str) -> Var {
    Var(Name(s.to_string()))
}

pub fn cons(s: &'static str, atoms: Vec<Atom>) -> Object {
    ObjCon(Constructor(s.to_string()), atoms)
}

pub trait E {
    fn exp(self) -> Expr;
}

impl E for Var {
    fn exp(self) -> Expr {
        EAtom(AtomVar(self))
    }
}
