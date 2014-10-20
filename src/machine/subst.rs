trait Subst<E, K, V> {
    fn subst(&self, expr: E, substitions: Vec<(K, V)>) -> E;
}


// impl subst::Subst<Expr, Atom, Atom> for Machine {
//     fn subst(&self, expr: Expr, bindings: Vec<(Atom, Atom)>) -> Expr {
//         let binding_set: HashSet<(Atom, Atom)> = bindings.iter_into().collect();
//
//         fn subst_with_set(expr: Expr, bindings: Set<(Atom, Atom)>) -> Expr {
//             expr /* match expr {
//                 EAtom(atom) =>
//             } */
//         }
//
//         fn in_set<S: Set<(K, V)>, K: PartialEq, V>(binding: S, key: K) -> bool {
//             let singleton_set: HashSet<(K, V)> = [key].iter_into().collect();
//             binding.contains(singleton_set)
//         }
//
//         subst_with_set(expr, binding_set)
//     }
// }
 
