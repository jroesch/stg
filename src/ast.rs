#[deriving(Clone, PartialEq, Eq, Show, Hash)]
pub struct Name(pub String);

#[deriving(Clone, PartialEq, Eq, Show, Hash)]
pub struct Var(pub Name);

#[deriving(Clone, PartialEq, Eq, Show, Hash)]
pub struct Constructor(pub String);

#[deriving(Clone, PartialEq, Show)]
pub enum Literal {
    IntLit(int),
    DoubleLit(f64)
}

#[deriving(Clone, PartialEq, Show)]
pub enum Atom {
    AtomLit(Literal),
    AtomVar(Var)
}

#[deriving(Clone, PartialEq, Show)]
pub enum PrimOp {
    Prim(String)
}

#[deriving(Clone, PartialEq, Show)]
pub enum Expr {
    EAtom(Atom),
    EFCall(u8, Var, Vec<Atom>),
    EPrimOp(PrimOp, Vec<Atom>),
    ELet(Var, Object, Box<Expr>),
    ECase(Box<Expr>, Vec<Alt>)
}

#[deriving(Clone, PartialEq, Show)]
pub enum Alt {
    AltCons(Constructor, Vec<Var>, Expr),
    AltName(Var, Expr)
}

#[deriving(Clone, PartialEq, Show)]
pub enum Object {
    ObjFun(Vec<Var>, Box<Expr>),
    ObjPap(Var, Vec<Atom>),
    ObjCon(Constructor, Vec<Atom>),
    ObjThunk(Box<Expr>),
    ObjAddr(u64), // This is an implementation detail.
    ObjBlackhole,
    Error
}

pub type Binding = (Var, Object);

#[deriving(Clone, PartialEq, Show)]
pub struct Program(pub Vec<Binding>);

//fn interpret(program: Program) -> Machine {}

#[deriving(Clone, PartialEq, Show)]
pub enum Kont {
    Case(Vec<Alt>),
    Update(Var),
    Apply(Vec<Atom>)
}
