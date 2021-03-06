use persistent::*;
use kinder::lift::Monoid;
use std::fmt::Debug;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};


pub trait TreeLike<V, A>: IntoIterator + Measurable<V, A>
    where V: Measure<A> + Debug,
          A: Clone + Measurable<V, A> + Debug
{
    fn len(&self) -> usize;
    fn to_tree(self) -> FingerTree<V, A>;

    fn pretty<'b, D>(&'b self, allocator: &'b D) -> DocBuilder<'b, D> where D: DocAllocator<'b>;

    /// Appends one finger tree to another
    fn append(self, other: Self) -> Self;

    /// Adds the given element to this tree as the first element.
    fn cons(self, t: A) -> FingerTree<V, A>;

    /// Folds the tree to the left with the given function and the given initial element
    fn fold_left<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(B) -> F2,
              F2: Fn(A) -> B;

    /// Folds the tree to the right with the given function and the given initial element
    fn fold_right<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(A) -> F2,
              F2: Fn(B) -> B;

    /// The first element of this tree
    fn head(self) -> (Option<A>, FingerTree<V, A>);

    fn lookup(&self, o: &Fn(V) -> usize, i: usize) -> (usize, Option<A>);

    //    ///Maps the given function across this tree, measuring with the given Measured instance.
    //    /// fn map<B, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> B;
    //    fn measure(self) -> (V, Self);

    /// Folds the tree to the left with the given function
    fn reduce_left<F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(A, A) -> A;

    /// Folds the tree to the right with the given function
    fn reduce_right<B, F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(B, B) -> A;

    /// Adds the given element to this tree as the first element
    fn snoc(self, t: A) -> FingerTree<V, A>;

    /// splits this tree into a pair of subtrees at the point where the given predicate,
    /// based on the measure, changes from false to true.
    fn split(self, f: &Fn(V) -> bool) -> (Self, Option<Self>) where Self: Sized;
}
