#![allow(dead_code)]
//! Persistent Finger Trees.

use algebra::ops::Mul;

pub trait Magma
{
    fn op(&self, rhs:&Self) -> Self;
}

impl<A> Magma for MgM<A>
    where
        A: Magma,
{
    #[inline]
    fn op(&self, rhs: &MgM<A>) -> MgM<A> {
        let &MgM(ref lhs) = self;
        let &MgM(ref rhs) = rhs;
        MgM(lhs.op(rhs))
    }
}


impl<A> Mul<MgM<A>> for MgM<A>
    where
        A: Magma,
{
    #[inline]
	type Output = MgM<A>;
    fn mul(self, rhs: MgM<A>) -> MgM<A> {
        self.op(&rhs)
    }
}


pub trait MagmaMultiplicative
    : Magma
    + Mul<Self> where Self: Sized {}

pub trait Semigroup: Magma
{
    #[inline]
    fn app(&self, rhs:&Self) -> Self where Self: Sized {
        self.op(rhs)
    }
}

pub trait Monoid: Semigroup
{
    fn nil() -> Self;
}


#[derive(Clone)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub struct MgM<A>(pub A);



pub use self::Digit::{
    One,
    Two,
    Three,
    Four,
};

pub use self::FingerTree::{
    Deep,
    Empty,
    Single,
};

#[derive(Clone)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub enum Digit<A> {
    One(A),
    Two(A,A),
    Three(A,A,A),
    Four(A,A,A,A),
}

impl<A> Digit<A> {
    fn fold_map<M,F>(&self, f:F) -> M
        where
            M:Monoid,
            F:Fn(&A) -> M,
    {
        match self {
            &One(ref a) => {
                f(a)
            },
            &Two(ref a, ref b) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b));
                res
            },
            &Three(ref a, ref b, ref c) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b)) * MgM(f(c));
                res
            },
            &Four(ref a, ref b, ref c, ref d) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b)) * MgM(f(c)) * MgM(f(d));
                res
            },
        }
    }
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
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

pub trait Measurable<V>
    where
        V:Monoid,
{
    fn measure(&self) -> V;
}

impl<V,A> Measurable<V> for Digit<A>
    where
        A:Measurable<V>,
        V:Monoid,
{
    fn measure(&self) -> V {
        self.fold_map(|d| { Measurable::measure(d) })
    }
}

impl<V,A> Measurable<V> for FingerTree<V,A>
    where
        A:Measurable<V>,
        V:Clone,
        V:Monoid,
{
    fn measure(&self) -> V {
        match self {
            &Empty => {
                Monoid::nil()
            },
            &Single(ref x) => {
                x.measure()
            },
            &Deep {
                measure:ref v,
                ..
            } => {
                (*v).clone()
            },
        }
    }
}

impl<V,A> FingerTree<V,A> {
    pub fn is_empty(&self) -> bool {
        match self {
            &Empty => {
                true
            },
            _ => {
                false
            }
        }
    }
}
