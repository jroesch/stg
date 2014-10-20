use ast::*;
use std::collections::hashmap::HashMap;
use std::collections::HashSet;
use pretty::*;

pub mod heap;
pub mod subst;
pub mod stack;
pub mod scope;

#[deriving(Show)]
pub struct Machine {
    static_bindings: Vec<Object>,
    heap: HashMap<u64, Object>,
    env: Vec<HashMap<Var, u64>>,
    stack: Vec<Kont>,
    counter: u64
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            static_bindings: Vec::new(),
            heap: HashMap::new(),
            env: vec![HashMap::new()],
            stack: Vec::new(),
            counter: 0
        }
    }

    fn current_env(&mut self) -> &mut HashMap<Var, u64> {
        self.env.get_mut(0)
    }

    // Extend the current environment with a binding.
    fn add_binding(&mut self, (var, obj): Binding) {
        let heap_addr = self.allocate(obj);
        self.current_env().insert(var, heap_addr);
    }

    fn lookup_addr(&self, var: &Var) -> u64 {
        for scope in self.env.iter() {
            match scope.find(var) {
                None => {},
                Some(addr) => return addr.clone()
            }
        }
        fail!("Variable not defined: {}", var);
    }

    fn fresh_addr(&mut self) -> u64 {
        let fresh_addr = self.counter;
        self.counter += 1;
        fresh_addr
    }

    pub fn interpret(mut self, Program(bindings): Program) {
        for binding in bindings.into_iter() {
            self.add_binding(binding);
        }

        let main_name = Var(Name("main".to_string()));

        let main_fn = EFCall(0, main_name, Vec::new());

        let expr = self.eval(main_fn);
        match expr {
            EAtom(AtomLit(lit)) =>
                println!("Result: {}", lit.pretty()),
            EAtom(AtomVar(ref var)) => {
                println!("Result: {}", self.lookup(var).pretty())
            },
            _ => fail!("Runtime error")
        };
    }
    /// What to do here?
    fn top(&mut self, handler: |Kont| -> Option<Expr>) -> Option<Expr> {
        self.stack.get(0).map(|k| {
            handler(k)
        });
    }

    fn eval(&mut self, e: Expr) -> Expr {
        match e {
            EAtom(v) => self.top(|k| match k {
                _ => None,
                Some(Case(alts)) => self.eval(ECase(v, alts))
            }),
            EAtom(AtomLit(lit)) => fail!(""),
            EAtom(AtomVar(ref y)) => match self.stack.pop() {
                None => self.invalid_state::<Expr>(),
                Some(Update(ref x)) => {
                    let x_addr = &self.lookup_addr(x);
                    let y_addr = self.lookup_addr(y);
                    // y_value should be a value i.e a
                    *(self.heap.get_mut(x_addr)) = ObjAddr(y_addr);
                    EAtom(AtomVar(y.clone()))
                },
                Some(head) => { self.stack.push(head); e.clone() }
            },
            EFCall(_, ref fun_name, ref args) => {
                match self.lookup(fun_name) {
                    &ObjFun(ref vars, ref expr) => {
                        let param_count = vars.len();
                        let arg_count = args.len();
                        if arg_count == param_count {

                        } else if arg_count > param_count {

                        } else {

                        }; *expr.clone()
                    },
                    &ObjPap(ref var, ref atoms) => fail!("pap"),
                    &ObjThunk(ref expr) => fail!("lol"),
                    _ => fail!("machine errror")
                }
            }

            EPrimOp(op, atoms) => { fail!("notimpl") },

            ELet(v, obj, exp) => {
                self.eval_in_scope(vec![(v, obj)], *exp)
            }

            ECase(EAtom(atom), alts) => {
                self.select_match(atom, alts)
            }

            ECase(expr, alts) => {
                self.stack.push(Case(alts));
                self.eval(e)
            }
        }
    }

    fn select_match(&mut self, atom: Atom, alts: Vec<Alt>) -> Expr {
        // for alt in alts {
        //     match atom {
        //         AtomLit(lit) => {
        //             // self.eval_in_scope(vec![])
        //             match alt {
        //                 AltName(var, body) =>
        //                     self.eval_in_scope(vec![(var, lit)])
        //             }
        //         }
        //
        //         AtomVar(var) => fail!("hello")
        //     };
        //
        //     match alt {
        //         AltName(name, body) => {
        //             self.eval_in_scope(
        //                 vec![name, expr],
        //                 *expr
        //             ),
        //         }
        //
        //         AltCons(cons, bindings, body) => {
        //             //match
        //         }
        //     }
        // }
        EAtom(atom)
    }

    fn invalid_state<E>(&self) -> E {
        fail!("invalid_state!")
    }

    fn eval_in_scope(&mut self, bindings: Vec<Binding>, exp: Expr) -> Expr {
        let mut binding_range = range(0u, bindings.len());

        self.env.push(HashMap::new());

        for binding in bindings.into_iter() {
            self.add_binding(binding);
        }

        let result = self.eval(exp);

        for _ in binding_range {
            self.env.pop();
        }

        return result;
    }
}

impl heap::Heap<u64, Object> for Machine {
    fn extend(&mut self, bindings: Vec<(u64, Object)>) {

    }

    fn value_for(&self, key: u64) -> Option<Object> {
        self.heap.find(&key).map(|k| k.clone())
    }

    fn allocate(&mut self, obj: Object) -> u64 {
        let addr = self.fresh_addr();
        self.heap.insert(addr, obj);
        addr
    }
}

impl scope::Scope for Machine {
    fn intro_scope(&mut self, bindings: Vec<(Atom, ValueOrAddr)>) -> Expr {

    }
}
