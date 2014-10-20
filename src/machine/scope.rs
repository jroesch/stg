use ast::{Atom, ValueOrAddr, Var, Expr};
use std::collections::hashmap::HashMap;

/// A mapping between named values whether atoms or heap addresses
trait Scope {
    fn intro_scope(&mut self, bindings: Vec<(Atom, ValueOrAddr)>, || -> Expr) -> Expr;
    fn current_scope(&mut self) -> &mut HashMap<Var, ValueOrAddr>;
    fn intro_binding(&mut self, binding: (Atom, ValueOrAddr));
}
