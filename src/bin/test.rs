extern crate fingertree;
use serde_json::value::ToJson;

use serde::Serialize;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
use serde_json::to_string_pretty;

use fingertree::{FingerTree, TreeLike, Measurable};

fn main() {


    let t: FingerTree<isize, char> = FingerTree::Empty;
    let t = t.cons('n')
        .cons('m')
        .cons('l')
        .cons('k')
        .cons('j')
        .cons('i')
        .cons('h')
        .cons('g')
        .cons('f')
        .cons('e')
        .cons('d')
        .cons('c')
        .cons('b')
        .cons('a');
    for item in t {
        println!("popped {:?}", item);
    }
    println!();

    let t: FingerTree<isize, char> = FingerTree::Empty;
    let t = t.snoc('a')
        .snoc('b')
        .snoc('c')
        .snoc('d')
        .snoc('e')
        .snoc('f');
        println!("{:?}",t);
//        .snoc('g');
//        .snoc('h')
//        .snoc('i')
//        .snoc('j')
//        .snoc('k')
//        .snoc('l')
//        .snoc('m')
//        .snoc('n');
    for item in t {
        println!("popped {:?}", item);
    }

}
