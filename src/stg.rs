#![feature(globs)]
use std::collections::hashmap::*;

#[deriving(Clone, PartialEq, Eq, Show, Hash, PartialOrd, Ord)]
struct Name(String);

#[deriving(Clone, PartialEq, Eq, Show, Hash)]
struct Var(Name);

struct Constructor(String);

enum Literal {
    IntLit(int),
    DoubleLit(f64)
}

enum Atom {
    AtomLit(Literal),
    AtomVar(Var)
}

enum PrimOp {
    Prim(String)
}

enum Expr {
    EAtom(Atom),
    EFCall(Var, Vec<Atom>),
    EPrimOp(PrimOp, Vec<Atom>),
    ELet(Var, Object, Box<Expr>),
    ECase(Box<Expr>, Vec<Alt>)
}

enum Alt {
    AltCons(Vec<Var>, Expr),
    AltName(Var, Expr)
}

enum Object {
    ObjFun(Vec<Expr>, Box<Expr>),
    ObjPap(Var, Vec<Atom>),
    ObjCon(Constructor, Vec<Atom>),
    ObjThunk(Box<Expr>),
    ObjBlackhole
}

type Binding = (Var, Object);

struct Program(Vec<Binding>);

type Heap = Vec<Object>;

//fn interpret(program: Program) -> Machine {}

enum Kont {
    Case(Vec<Alt>),
    Update(Object),
    Apply(Vec<Atom>)
}

struct Machine {
    static_bindings: Vec<Object>,
    heap: Vec<Object>,
    env: Vec<HashMap<Var, uint>>,
    stack: Vec<Kont>
}

impl Machine {
    fn new() -> Machine {
        Machine {
            static_bindings: Vec::new(),
            heap: Vec::new(),
            env: vec![HashMap::new()],
            stack: Kont::new()
        }
    }

    fn current_env(&self) -> &HashMap<Var, uint> {
        self.env.get(0u)
    }

    fn interpret(self, Program(bindings): Program) {
        for binding in bindings.iter() {
            add_binding(binding);
        }

        let main_fn = self.lookup(Var(Name("main")));

        self.eval(main_fn)
    }

    fn eval(&self, e: Expr) -> Value {

    }
    // Extend the current environment with a binding.
    fn add_binding(&mut self, (var, obj): Binding) {
        let heap_addr = self.allocate(obj);
        self.current_env().insert(var, heap_addr);
    }

    /// Allocate an object on the Heap.
    /// Allocation strategy is really shitty atm.
    /// We never collect.
    fn allocate(&mut self, obj: Object) -> int {
        self.heap.push(obj);
    }

    fn lookup(&self, var: Var) -> &Object {
        for scope in self.env.iter() {
            match scope.find(&var) {
                None => {},
                Some(addr) => return self.heap.get(*addr)
            }
        }
        fail!("Variable not defined: {}", var);
    }
}

// fn subst(x: Var, xprime: Var, e: Expr) -> Expr {
//     match e {
//         EAtom(atom) => EAtom(subst_atom(x, xprime, atom)),
//         EFCall(f, args) => {
//             if f == x {
//                 EFCall(xprime.clone(), args.move_iter().map(|arg|
//                     subst_atom(x.clone(), xprime.clone(), arg)).collect()
//                 )
//             } else {
//                 EFCall(f, args)
//             }
//         },
//         EPrimOp(op, args) => fail!("Yolo!"),
//         ELet(name, obj, expr) => fail!("Yolo!"),
//         ECase(expr, alts) => fail!("Yolo!"),
//     }
// }
//
// fn subst_atom(x: Var, xprime: Var, a: Atom) -> Atom {
//     match a {
//         AtomVar(v) => if x == v { AtomVar(xprime) } else { AtomVar(v) },
//         _ => a
//     }
// }
