#![crate_name="fingertree"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://darinmorrison.github.io/fingertree.rs/doc/fingertree/")]

//! This crate implements the Finger Tree data type.

#![allow(dead_code)]
#![feature(struct_variant)]

extern crate algebra;

use algebra::{
    M,
    Monoid,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum Digit<A> {
    One(A),
    Two(A,A),
    Three(A,A,A),
    Four(A,A,A,A),
}

impl<A> Digit<A> {
    fn fold_map<M>(&self, f:|&A| -> M) -> M
        where
            M:Monoid,
    {
        match self {
            &One(ref a) => {
                f(a)
            },
            &Two(ref a, ref b) => {
                let M(res) = M(f(a)) * M(f(b));
                res
            },
            &Three(ref a, ref b, ref c) => {
                let M(res) = M(f(a)) * M(f(b)) * M(f(c));
                res
            },
            &Four(ref a, ref b, ref c, ref d) => {
                let M(res) = M(f(a)) * M(f(b)) * M(f(c)) * M(f(d));
                res
            },
        }
    }
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum FingerTree<V,A> {
    Empty,
    Single(A),
    Deep {
        measure: V,
        prefix:  Digit<A>,
        tree:    Box<FingerTree<V,A>>,
        suffix:  Digit<A>,
    },
}
