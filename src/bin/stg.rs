#![feature(globs)]
extern crate stg;

use stg::*;
use stg::dsl::*;
use stg::ast::*;
use stg::machine::*;

fn main() {
    let machine = Machine::new();
    let nil: Binding = (
        var("nil"), cons("Nil", vec![])
    );

    let prog = Program(vec![
        nil,
        (var("main"), ObjFun(Vec::new(), box var("nil").exp()))
    ]);

    machine.interpret(prog)
}
