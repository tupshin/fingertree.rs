#![crate_name="fingertree"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://darinmorrison.github.io/fingertree.rs/doc/fingertree/")]

//! This crate implements the Finger Tree data type.

#![allow(dead_code)]
#![feature(struct_variant)]

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
enum Digit<A> {
    One(A),
    Two(A,A),
    Three(A,A,A),
    Four(A,A,A,A),
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
enum FingerTree<V,A> {
    Empty,
    Single(A),
    Deep {
        measure: V,
        prefix:  Digit<A>,
        tree:    Box<FingerTree<V,A>>,
        suffix:  Digit<A>,
    },
}

mod rope {
    use super::{
        Deep,
        Digit,
        Empty,
        FingerTree,
        Four,
        One,
        Single,
        Three,
        Two,
    };

    struct Offset(i32);
    struct Chunk(Vec<u8>);
    struct Body(FingerTree<Offset,Chunk>);
    struct Rope(Body);
}
