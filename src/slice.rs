//! Support for encoding fixed length slices

use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};

use std::mem;
use std::ops::Deref;
use std::cmp::PartialEq;

/// A struct for encoding fixed length arrays
/// 
/// ```ignore rust
/// #[derive(RustcEncodable, RustcDecodable)]
/// struct Foo {
///     magic : Fixed<[u8; 4]>,
///     data  : Fixed<[u32; 2]>
/// }
/// ```
/// 
/// Each array ends up encoded as if you had listed N elements seperately, for example:
///
/// ```ignore rust
/// #[derive(RustcEncodable, RustcDecodable)]
/// struct Foo {
///     magic1 : u8,
///     magic2 : u8,
///     magic3 : u8,
///     magic4 : u8,
/// }
/// ```
/// 
/// This may not work properly with some encoders that require structuring around multiple
/// elements.
#[derive(Clone, Debug)]
pub struct Fixed<T>(pub T);

impl<T> Deref for Fixed<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        &self.0
    }
}

impl<T: PartialEq> PartialEq for Fixed<T> {
    fn eq(&self, other: &Fixed<T>) -> bool {
        self.0.eq(&other.0)
    }
}

macro_rules! def_fixed {
    ($( $n:expr )*) => {$(
        impl<T : Decodable> Decodable for Fixed<[T; $n]> {
            fn decode<D: Decoder>(d: &mut D) -> Result<Fixed<[T; $n]>, D::Error> {
                /* this usage is only safe because we're sure to either:
                 * - fill the entire slice, and return it
                 * - OR return an error discarding the slice
                 */
                let mut v : [T; $n] = unsafe { mem::uninitialized() };
                for i in 0..$n {
                   v[i] = try!(Decodable::decode(d));
                }

                Ok(Fixed(v))
            }
        }

        impl<T : Encodable> Encodable for Fixed<[T; $n]> {
            fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                for i in 0..$n {
                    try!(self.0[i].encode(s));
                }
                Ok(())
            }
        }
    )*}
}

def_fixed! {0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
    32}
