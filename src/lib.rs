#![allow(dead_code)]
#![feature(try_from)]

#[macro_use]
extern crate eko_gc_derive;
#[macro_use]
extern crate err_derive;

pub mod compiler;
pub mod core;
pub mod engine;
