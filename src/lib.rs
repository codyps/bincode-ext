#![feature(core)]

extern crate "rustc-serialize" as rustc_serialize;

pub mod byte_order;
pub mod slice;

#[cfg(test)] mod test;
