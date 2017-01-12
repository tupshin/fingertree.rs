#![allow(dead_code)]
//! Persistent Finger Trees.

mod digit;
mod measure;
mod fingertree;
mod deep;
mod treelike;

pub use self::treelike::*;
pub use self::digit::*;
pub use self::fingertree::*;
pub use self::measure::*;
pub use self::deep::*;
