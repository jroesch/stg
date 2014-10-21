// use super::super::ast;
//
// trait Subst<K, V> {
//     fn subst(&self, substitions: Vec<(K, V)>) -> Self;
// }
//
// struct SubstMap {
//     hash_map: HashMap<Atom, Atom>
// }
//
// impl SubstMap {
//     fn new() -> SubstMap {
//         SubstMap { hash_map: HashMap::new() }
//     }
//
//     fn sub(&self, key: Atom) -> Atom {
//         self.lookup(&key).unwrap_or_else(key)
//     }
//
//     // fn sub_var(&self key: Var) -> Var {
//     //     self.lookup(&key).map(|r|
//     //         match r {
//     //             AtomVar(v) => v,
//     //             _ => fail!("this is a bug!")
//     //         }
//     //     ).unwrap_or_else(key)
//     // }
//
//     fn add(&mut self, bindings: Vec<(Atom, Atom)>) -> SubstMap {
//         for (key, value) in bindings {
//             self.hash_map.insert(&k, value)
//         }
//     }
//
//     fn remove(&mut self, keys: Vec<Atom>) -> SubstMap {
//         let mut internal_map = self.hash_map.clone();
//         for key in keys {
//             internal_map.remove(key)
//         };
//         SubstMap { hash_map: internal_map }
//     }
// }
//
// impl subst::Subst<Atom, Atom> for Expr {
//     fn subst(&self, bindings: Vec<(Atom, Atom)>) -> Expr {
//         let binding_set = SubstMap::new().add(bindings);
//
//         fn subst_with_set(expr: Expr, substs: SubstMap) -> Expr {
//             match expr {
//                 EAtom(atom) => substs.sub(atom),
//                 ELet(var, object, box_expr) => ELet(
//                     var,
//                     object,
//                     subst_with_set(box_expr, substs.remove(vec![var]))
//                 ),
//                 EFCall(arity, var, atoms) =>
//                     EFCall(arity, substs.sub(var), atoms.map(|a| {
//                         subst.sub(a)
//                     }).collect()),
//                 EPrimOp(prim_op, atoms) => {
//                     let sub_atoms = atoms.map(|a| {
//                         substs.sub(a)
//                     }).collect();
//
//                     EPrimOp(sub_atoms)
//                 }
//
//                 ECase(expr, alts) => {
//                     let sub_expr = subst_with_set(expr, substs);
//                     let sub_alts = alts.map(|alt| {
//                         match alt {
//                             AltCons(cons, vars, expr) => {
//                                 AltCons(cons, vars, subst_with_set(expr, subst.remove(vars)))
//                             }
//                             AltName(var, expr) => {
//                                 subst_with_set(expr, substs.remove(var))
//                             }
//                         }
//                     }).collect();
//                     ECase(sub_expr, sub_alts)
//                 }
//             }
//         }
//
//         subst_with_set(expr, binding_set)
//     }
// }
