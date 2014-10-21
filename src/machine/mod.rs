use ast::*;
use std::collections::hashmap::HashMap;
use std::collections::HashSet;
use pretty::*;
use self::heap::Heap;
use self::scope::Scope;
//pub use self::stack::

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

impl Heap<u64, Object> for Machine {
    fn extend(&mut self, bindings: Vec<(u64, Object)>) {
        for (addr, obj) in bindings.into_iter() {
            self.heap.insert(addr, obj);
        }
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

impl Scope for Machine {
    fn enter_scope(&mut self) {
        self.env.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.env.pop();
    }

    fn current_scope(&mut self) -> &mut HashMap<Var, u64> {
        self.env.get_mut(0)
    }

    fn intro_binding(&mut self, name: Var, addr: u64) {
        self.current_scope().insert(name, addr);
    }

    fn lookup(&self, var: &Var) -> u64 {
        for scope in self.env.iter() {
            match scope.find(var) {
                None => {},
                Some(addr) => return addr.clone()
            }
        }
        fail!("Variable not defined: {}", var);
    }

    fn eval(&mut self, e: Expr) -> Expr {
        match e.clone() {
            ELet(x, obj, exp) => {
                debug!("Rule: LET")
                let x_prime = self.allocate(obj);
                self.in_scope(vec![(x, x_prime)], *exp)
            }

            ECase(scrutinee, alts) => {
                debug!("Rule: LET");
                e.clone()
            },

            EAtom(atom) => {
                match self.current_kont() {
                    Some(Case(alts)) => {
                        let _ = self.stack.pop();
                        let value = match atom {
                            AtomLit(lit) => AtomLit(lit),
                            AtomVar(var) => { self.value_for(self.lookup(&var)); AtomVar(var) }
                        };
                        self.eval(ECase(box EAtom(value), alts))
                    }

                    Some(Update(x)) => match atom {
                        AtomVar(y) => {
                            debug!("UPDATE");
                            let _ = self.stack.pop();
                            let y_addr = self.lookup(&y);
                            self.intro_binding(x, y_addr);
                            EAtom(AtomVar(y))
                        },
                        _ => { fail!("not impl") }
                    },

                    _ => { fail!("not impl") }
                }
            }

            EFCall(..) => {e},

            EPrimOp(..) => fail!("Ignored for now.")
        }
    }
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

    pub fn interpret(mut self, Program(bindings): Program) {
        for (name, object) in bindings.into_iter() {
            let addr = self.allocate(object);
            self.intro_binding(name, addr);
        }

        let main_name = Var(Name("main".to_string()));

        let main_fn = EFCall(0, main_name, Vec::new());

        let expr = self.eval(main_fn);
        match expr {
            EAtom(AtomLit(lit)) =>
                println!("Result: {}", lit.pretty()),
            EAtom(AtomVar(ref var)) => {
                println!("Result: {}", self.value_for(self.lookup(var)).unwrap().pretty())
            },
            _ => fail!("Runtime error")
        };
    }

    // Utils

    fn fresh_addr(&mut self) -> u64 {
        let fresh_addr = self.counter;
        self.counter += 1;
        fresh_addr
    }

    fn current_kont(&self) -> Option<Kont> {
        if self.stack.len() >= 1 {
            Some(self.stack.get(0).clone())
        } else { None }
    }
}
