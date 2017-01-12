use persistent::*;
use kinder::lift::Monoid;
use std::ops::{Add, Mul};

#[derive(Ord,PartialOrd,PartialEq,Eq,Debug,Clone)]
pub struct DeepTree<V, A>
    where A: Clone
{
    measure: V,
    prefix: Digit<A>,
    middle: Box<FingerTree<V, A>>,
    suffix: Digit<A>,
}

impl<V, A> Measurable<V, A> for DeepTree<V, A>
    where V: Measure<A>,
          A: Measurable<V, A> + Clone
{
    fn to_tree(self) -> FingerTree<V, A> {
        unimplemented!()
    }
    fn measure(&self) -> V {
        self.measure.clone()
    }
}


impl<V, A> Add for DeepTree<V, A>
    where A: Clone + Measurable<V, A>,
          V: Measure<A>
{
    type Output = DeepTree<V, A>;
    fn add(self, deep: DeepTree<V, A>) -> Self {
        let m = self.measure;
        DeepTree {
            measure: m.clone() + deep.measure,
            prefix: self.prefix,
            middle: Box::new(DeepTree::add_digits_0(m,
                                                    self.middle.as_ref().clone(),
                                                    self.suffix,
                                                    deep.prefix,
                                                    deep.middle.as_ref().clone())),
            suffix: deep.suffix,
        }
        //      deep -> new Deep<>(m, m.sum(measure(), deep.measure()), prefix,
        //        addDigits0(m, middle, suffix, deep.prefix, deep.middle), deep.suffix)
    }
}

impl<V, A> PossiblyEmpty for DeepTree<V, A>
    where A: Clone
{
    fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

impl<V, A> Iterator for DeepTree<V, A>
    where A: Clone
{
    type Item = A;
    fn next(&mut self) -> Option<A> {
        unimplemented!()
    }
}


impl<V, A> DeepTree<V, A>
    where A: Clone + Measurable<V, A>,
          V: Measure<A>
{
    ///   Constructs a deep tree. This structure consists of two digits,
    /// of 1 to 4 elements each, on the left and right,
    ///   with the rest of the tree in the middle.
    ///
    ///   @paramost elements of the tree.
    ///   @param middle The subtree, which is a Finger Tree of 2-3 nodes.
    ///   @param suffix The rightmost elements of the tree.
    ///   @return A new finger tree with the given prefix, suffix, and middle.
    ///
    ///  public FingerTree<V, A> deep(final Digit<V, A> prefix,
    /// final FingerTree<V, Node<V, A>> middle,
    ///                               final Digit<V, A> suffix) {
    ///    return deep(m.sum(prefix.measure(), m.sum(middle.measure(), suffix.measure())),
    /// prefix, middle, suffix);
    ///  }
    pub fn new(prefix: Digit<A>, middle: FingerTree<V, A>, suffix: Digit<A>) -> FingerTree<V, A> {
        FingerTree::Deep(DeepTree {
            measure: prefix.measure() + middle.measure() + suffix.measure(),
            prefix: prefix,
            middle: Box::new(middle),
            suffix: suffix,
        })
    }
    fn add_digits_0<M: Measurable<V, A>>(m: M,
                                         m1: FingerTree<V, A>,
                                         s1: Digit<A>,
                                         p2: Digit<A>,
                                         m2: FingerTree<V, A>)
                                         -> FingerTree<V, A> {
        let mk = m.to_tree();
        match s1 {
            Digit::One(a) => unimplemented!(),
            Digit::Two(a, b) => unimplemented!(),
            Digit::Three(a, b, c) => unimplemented!(),
            Digit::Four(a, b, c, d) => unimplemented!(),

        }
    }
    //        private static <V, A> FingerTree<V, Node<V, A>> addDigits0(
    //            final Measured<V, A> m, final FingerTree<V, Node<V, A>> m1,
    //            final Digit<V, A> s1, final Digit<V, A> p2,
    //            final FingerTree<V, Node<V, A>> m2) {
    //
    //        final MakeTree<V, A> mk = mkTree(m);
    //        return s1.match(
    //            one1 -> p2.match(
    //                one2 -> append1(m, m1, mk.node2(one1.value(), one2.value()), m2),
    //                two2 -> {
    //                    final V2<A> vs = two2.values();
    //                    return append1(m, m1, mk.node3(one1.value(), vs._1(), vs._2()), m2);
    //                },
    //                three -> {
    //                    final V3<A> vs = three.values();
    //                    return append2(m, m1, mk.node2(one1.value(), vs._1()), mk.node2(vs._2(), vs._3()), m2);
    //                },
    //                four -> {
    //                    final V4<A> vs = four.values();
    //                    return append2(m, m1, mk.node3(one1.value(), vs._1(), vs._2()), mk.node2(vs._3(), vs._4()), m2);
    //                }
    //            ),
    //            two1 -> {
    //                final V2<A> v1 = two1.values();
    //                return p2.match(
    //                    one -> append1(m, m1, mk.node3(v1._1(), v1._2(), one.value()), m2),
    //                    two2 -> {
    //                        final V2<A> v2 = two2.values();
    //                        return append2(m, m1, mk.node2(v1._1(), v1._2()), mk.node2(v2._1(), v2._2()), m2);
    //                    },
    //                    three -> {
    //                        final V3<A> v2 = three.values();
    //                        return append2(m, m1, mk.node3(v1._1(), v1._2(), v2._1()), mk.node2(v2._2(), v2._3()), m2);
    //                    },
    //                    four -> {
    //                        final V4<A> v2 = four.values();
    //                        return append2(m, m1, mk.node3(v1._1(), v1._2(), v2._1()), mk.node3(v2._2(), v2._3(), v2._4()), m2);
    //                    }
    //                );
    //            },
    //            three1 -> {
    //                final V3<A> v1 = three1.values();
    //                return p2.match(
    //                    one -> append2(m, m1, mk.node2(v1._1(), v1._2()), mk.node2(v1._3(), one.value()), m2),
    //                    two -> {
    //                        final V2<A> v2 = two.values();
    //                        return append2(m, m1, mk.node3(v1), mk.node2(v2), m2);
    //                    },
    //                    three2 -> append2(m, m1, mk.node3(v1), mk.node3(three2.values()), m2),
    //                    four -> append3(m, m1, mk.node3(v1),
    //                        mk.node2(four.values()._1(), four.values()._2()),
    //                        mk.node2(four.values()._3(), four.values()._4()), m2
    //                    )
    //                );
    //            },
    //            four1 -> {
    //                final V4<A> v1 = four1.values();
    //                return p2.match(
    //                    one -> append2(m, m1, mk.node3(v1._1(), v1._2(), v1._3()), mk.node2(v1._4(), one.value()), m2),
    //                    two -> {
    //                        final V2<A> v2 = two.values();
    //                        return append2(m, m1, mk.node3(v1._1(), v1._2(), v1._3()), mk.node3(v1._4(), v2._1(), v2._2()), m2);
    //                    },
    //                    three -> {
    //                        final V3<A> v2 = three.values();
    //                        return append3(m, m1, mk.node3(v1._1(), v1._2(), v1._3()), mk.node2(v1._4(), v2._1()), mk.node2(v2._2(), v2._3()), m2);
    //                    },
    //                    four2 -> {
    //                        final V4<A> v2 = four2.values();
    //                        return append3(m, m1, mk.node3(v1._1(), v1._2(), v1._3()), mk.node3(v1._4(), v2._1(), v2._2()), mk.node2(v2._3(), v2._4()), m2);
    //                    }
    //                );
    //            }
    //        );
    //    }
}

impl<V, A> TreeLike<V, A> for DeepTree<V, A>
    where V: Measure<A>,
          A: Measurable<V, A> + Clone
{
    /// Appends one finger tree to another
    fn append(self, other: Self) -> Self {
        unimplemented!()
    }

    /// Adds the given element to this tree as the first element.
    fn cons(self, t: A) -> Self {
        unimplemented!()
    }

    /// Folds the tree to the left with the given function and the given initial element
    fn fold_left<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(B) -> F2,
              F2: Fn(A) -> B
    {
        unimplemented!()
    }

    /// Folds the tree to the right with the given function and the given initial element
    fn fold_right<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(A) -> F2,
              F2: Fn(B) -> B
    {
        unimplemented!()
    }

    /// The first element of this tree
    fn head(self) -> (Option<A>, Self) {
        unimplemented!()
    }

    fn lookup(self, o: &Fn(V) -> usize, i: usize) -> (usize, Option<A>) {
        unimplemented!()
    }

    //    ///Maps the given function across this tree, measuring with the given Measured instance.
    //    /// fn map<B, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> B{}
    //    fn measure(self) -> (V, Self){}

    /// Folds the tree to the left with the given function
    fn reduce_left<F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(A, A) -> A
    {
        unimplemented!()
    }

    /// Folds the tree to the right with the given function
    fn reduce_right<B, F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(B, B) -> A
    {
        unimplemented!()
    }

    /// Adds the given element to this tree as the first element
    fn snoc(self, t: A) -> Self {
        unimplemented!()
    }

    /// splits this tree into a pair of subtrees at the point where the given predicate,
    /// based on the measure, changes from false to true.
    fn split(self, f: &Fn(V) -> bool) -> (Self, Option<Self>)
        where Self: Sized
    {
        unimplemented!()
    }
}
