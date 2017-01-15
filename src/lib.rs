#![feature(proc_macro)]
#![crate_name="fingertree"]
#![crate_type="lib"]

#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/fingertree.rs/doc/fingertree/")]

//! This crate implements the Finger Tree data type.

// license = "MIT"

#[macro_use]
extern crate kinder;


#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate serde_json;
extern crate pretty;

pub mod persistent;

pub use persistent::*;
