#![feature(core)]

extern crate "rustc-serialize" as rustc_serialize;

pub mod byte_order;
pub mod slice;

pub use slice::Fixed;
pub use byte_order::Le;

#[cfg(test)] mod test;
