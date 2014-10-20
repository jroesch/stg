use std::io::{IoResult, MemWriter};
use std::fmt::{rt, Show};
use core::fmt::{FormatWriter};
use ast::*;

pub struct Printer<'a> {
    /// Character used as 'fill' whenever there is alignment
    pub fill: char,
    /// Boolean indication of whether the output should be left-aligned
    pub align: rt::Alignment,
    /// Optionally specified integer width that the output should be
    pub width: Option<uint>,
    /// Optionally specified precision for numeric types
    pub precision: Option<uint>,
    pub buf: &'a mut Writer + 'a,
}

impl<'a> Printer<'a> {
    fn write(&mut self, bytes: &[u8]) -> IoResult<()> {
        self.buf.write(bytes)
    }
}

pub trait PrettyPrintable: Show {
    fn pretty(&self) -> String {
        let mut writer = MemWriter::new();

        // this is a very clever way to program the borrow system to
        // our favor.

        // Prior the data structure lives to
        (|| {
            let w = &mut writer;
            let mut printer = Printer {
                fill: ' ',
                width: Some(80),
                precision: None,
                buf: w,
                align: rt::AlignLeft,
            };

            let _ = self.pretty_with_printer(&mut printer);
        })();

        String::from_utf8(writer.unwrap()).unwrap()
    }

    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()>;
}

impl PrettyPrintable for Object {
    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()> {
        match self {
            &ObjCon(ref cons, ref args) => {
                cons.pretty();
                let _ = printer.write("(".as_bytes());
                for arg in args.iter() {
                    arg.pretty();
                }
                try!(printer.write(")".as_bytes()));
            },
            _ => { try!(printer.write("not impl".as_bytes())) }
        };
        Ok(())
    }
}

impl PrettyPrintable for Expr {
    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()> {
        match self {
            &EAtom(ref atom) => {
                try!(atom.pretty_with_printer(printer));
            }

            _ => { try!(printer.write("not impl".as_bytes())) }
        }; Ok(())
    }
}

impl PrettyPrintable for Literal {
    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()> {
        match self {
            &IntLit(ref i) => { printer.write(i.to_string().as_bytes()); },
            _ => { printer.write("not impl".as_bytes()); } //&DoubleLit
        };
        Ok(())
    }
}

impl PrettyPrintable for Atom {
    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()> {
        printer.write("AnAtom".as_bytes())
    }
}

impl PrettyPrintable for Constructor {
    fn pretty_with_printer<'a>(&self, printer: &mut Printer<'a>) -> IoResult<()> {
        let &Constructor(ref name) = self;
        printer.write(name.as_bytes())
    }
}

//impl Pretty
