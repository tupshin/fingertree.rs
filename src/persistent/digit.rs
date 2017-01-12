use kinder::lift::Monoid;
use std::ops::{Mul, Add};
use persistent::*;

#[derive(Clone,Eq,Ord,PartialEq,PartialOrd,Debug)]
pub enum Digit<A> {
    One(A),
    Two(A, A),
    Three(A, A, A),
    Four(A, A, A, A),
}

use self::Digit::*;
impl<A> Digit<A> {
    pub fn fold_map<M, F>(&self, f: F) -> M
        where M: Monoid + Mul<Output = M>,
              F: Fn(&A) -> M
    {
        match *self {
            One(ref a) => f(a),
            Two(ref a, ref b) => f(a) * f(b),
            Three(ref a, ref b, ref c) => f(a) * f(b) * f(c),
            Four(ref a, ref b, ref c, ref d) => f(a) * f(b) * f(c) * f(d),
        }
    }

    pub fn new(a: A) -> Self {
        Digit::One(a)
    }
}

impl<V, A> Measurable<V, A> for Digit<A>
    where A: Measurable<V, A> + Clone,
          V: Measure<A>
{
    fn to_tree(self) -> FingerTree<V, A> {
        unimplemented!()
    }

    fn measure(&self) -> V {
        self.fold_map(|d| Measurable::measure(d))
    }
}
