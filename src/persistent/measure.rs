use kinder::lift::Monoid;
use std::ops::{Add, Mul};
use std::fmt::Debug;

use persistent::*;

pub trait Measurable<V, A>
    where V: Measure<A>,
          A: Measurable<V, A> + Clone,
          Self: Sized
{
    fn measure(&self) -> V;
    // fn to_tree(self) -> FingerTree<V, A>;
}

impl Measure<isize> for isize {}
impl Measurable<isize, isize> for isize {
    fn measure(&self) -> isize {
        self.clone()
    }
}

impl Measurable<isize, char> for char {
    fn measure(&self) -> isize {
        1
    }
}

impl Measure<char> for isize {}
impl Measurable<isize, char> for isize {
    fn measure(&self) -> isize {
        self.clone()
    }
}

impl Measurable<isize, i32> for i32 {
    fn measure(&self) -> isize {
        1
    }
}

impl Measure<i32> for isize {}
impl Measurable<isize, i32> for isize {
    fn measure(&self) -> isize {
        self.clone()
    }
}
// pub struct Measured<V>(V);
//
// impl<V> Measurable<V> for Measured<V> where V:Monoid{
// 	fn measure(&self) -> V {
// 		self.0
// 	}
//

pub trait Measure<A>
    : Monoid + Default + Clone + Add<Output = Self> + Measurable<Self, A> +
    Mul<Output = Self>+PartialOrd
    where A: Clone + Measurable<Self, A>
{
}


impl<'a, V, A: 'a> Measurable<V, A> for FingerTree<V, A>
    where A: Measurable<V, A> + Clone + Debug,
          V: Measure<A> + Debug
{
    fn measure(&self) -> V {
        match *self {
            FingerTree::Empty => V::default(),
            FingerTree::Single(ref x) => x.measure(),
            FingerTree::Deep(ref d) => d.measure().clone(),
        }
    }
}
