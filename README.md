# Extentions for Bincode (Binary Encoder / Decoder)

[![Build Status](https://travis-ci.org/jmesmon/bincode-ext.svg)](https://travis-ci.org/jmesmon/bincode-ext)

## Example

```rust
extern crate bincode;
extern crate bincode_ext;

use bincode_ext::byte_order::Le;

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct Foo {
	x: Le<u32>, // little endian
	y: u32,     // big endian, due to defaults in bincode
}
