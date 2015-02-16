extern crate bincode;

use rustc_serialize::Encodable;
use super::byte_order::Le;

#[derive(RustcDecodable, RustcEncodable, PartialEq, PartialOrd, Debug)]
struct X {
    v: Le<u16>,
    o: u16,
}

#[test]
fn test_le() {
    let v = 53343;
    let x = X { v: Le(v), o: v };
    let e : Vec<u8> = bincode::encode(&x, bincode::SizeLimit::Infinite).unwrap();
    assert_eq!(e, [95, 208, 208, 95]);
    let d : X = bincode::decode(&e).unwrap();
    assert_eq!(x, d);
    println!("{:?} == {:?} == {:?}", x, e, d);
}
