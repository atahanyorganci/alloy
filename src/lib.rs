#[deny(missing_debug_implementations)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate structopt;
#[macro_use]
extern crate phf;

pub mod ast;
pub mod compiler;
pub mod object;
pub mod parser;
