//! Provide a way to emit Little Endian bytes from bincode
//! Note that this depends upon bincode trying to emit big endian bytes.

use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use std::num::Int;
use std::ops::Deref;

/// Use to represent a little endian integer.
// XXX: this sample breaks because doc-tests is a POS.
//     extern crate "rustc-serialize" as rustc_serialize;
//     extern crate bincode_ext;
//     #[derive(RustcDecodable,RustcEncodable)]
//     struct Foo {
//         is_be: u16,
//         is_le: bincode_ext::byte_order::Le<u16>
//     }
//
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Le<T>(pub T);

macro_rules! define_le {
    ($int:ty, $read:ident, $emit:ident) => {
        impl Decodable for Le<$int> {
            fn decode<D: Decoder>(d: &mut D) -> Result<Le<$int>, D::Error> {
                Ok(Le(try!(d.$read()).swap_bytes()))
            }
        }

        impl Encodable for Le<$int> {
            fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                s.$emit(self.0.swap_bytes())
            }
        }
    }
}

define_le! { u16, read_u16, emit_u16 }
define_le! { u32, read_u32, emit_u32 }
define_le! { u64, read_u64, emit_u64 }

impl<T> Deref for Le<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.0
    }
}
