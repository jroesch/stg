use ast::{Atom, Var, Expr};
use std::collections::hashmap::HashMap;

/// A mapping between named values whether atoms or heap addresses
pub trait Scope {
    /* This is hacky, how to modularize the interface */
    fn eval(&mut self, e: Expr) -> Expr;

    fn in_scope(&mut self, bindings: Vec<(Var, u64)>, expr: Expr) -> Expr {
        let mut binding_range = range(0u, bindings.len());

        self.enter_scope();

        for (name, addr) in bindings.into_iter() {
            self.intro_binding(name, addr);
        }

        let result = self.eval(expr);

        self.exit_scope();

        return result;
    }

    fn lookup(&self, var: &Var) -> u64;
    fn enter_scope(&mut self);
    fn exit_scope(&mut self);
    fn current_scope(&mut self) -> &mut HashMap<Var, u64>;
    fn intro_binding(&mut self, name: Var, addr: u64);
}
