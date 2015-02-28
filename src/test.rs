extern crate bincode;

use rustc_serialize::Encodable;
use super::byte_order::Le;

#[derive(RustcDecodable, RustcEncodable, PartialEq, PartialOrd, Debug)]
struct X {
    v: Le<u16>,
    o: u16,
}

#[test]
fn byte_order_le() {
    let v = 53343;
    let x = X { v: Le(v), o: v };
    let e : Vec<u8> = bincode::encode(&x, bincode::SizeLimit::Infinite).unwrap();
    assert_eq!(e, [95, 208, 208, 95]);
    let d : X = bincode::decode(&e).unwrap();
    assert_eq!(x, d);
    println!("{:?} == {:?} == {:?}", x, e, d);
}

#[test]
fn slice_fixed() {
    use super::slice::Fixed;
    #[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
    struct Y {
        v: Fixed<[u8; 3]>,
        y: Fixed<[u32; 2]>
    }

    let x = Y { v : Fixed([2, 4, 1]), y: Fixed([5,6]) };
    let e : Vec<u8> = bincode::encode(&x, bincode::SizeLimit::Infinite).unwrap();
    assert_eq!(e, [2, 4, 1, 0, 0, 0, 5, 0, 0, 0, 6]);
    let d : Y = bincode::decode(&e).unwrap();
    assert_eq!(x, d);
    println!("{:?} == {:?} == {:?}", x, e, d);
}
