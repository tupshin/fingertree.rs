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
    //    println!("{:?}", t);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    //   println!("popped {:?} leaving\n {:?}", a,t);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!(//    println!("{:?}", t);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    //   println!("popped {:?} leaving\n {:?}", a,t);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    println!("{:?}", t);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    let (a, t) = t.head();
    //    println!("popped {:?}", a);
    //    println!("{:?}", t);

    //    let t = t.snoc('g')
    //        .snoc('f')
    //        .snoc('e')
    //        .snoc('d')
    //        .snoc('c')
    //        .snoc('b')
    //        .snoc('a');
    //    println!("{:?}", to_string_pretty(&t).unwrap());
    //    let (a, t) = t.head();
    //    println!("popped {:?} from {}", a, to_string_pretty(&t).unwrap());
    //    let (a, t) = t.head();
    //    println!("popped {:?} from {}", a, to_string_pretty(&t).unwrap());
    //    let (a, t) = t.head();
    //    println!("popped {:?} from {}", a, to_string_pretty(&t).unwrap());
    //    let (a, t) = t.head();
    //    println!("popped {:?} from {}", a, to_string_pretty(&t).unwrap());

}
