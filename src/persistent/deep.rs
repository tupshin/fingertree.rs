use persistent::*;
use std::ops::Add;
use Digit::*;
use std::fmt::Debug;
use std::fmt;
use serde::ser::{Serializer, Serialize};
use pretty::{BoxAllocator, DocAllocator, DocBuilder};

#[derive(Ord,PartialOrd,PartialEq,Eq,Debug,Clone)]
pub struct DeepTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    m: V,
    prefix: Digit<V, A>,
    middle: Box<FingerTree<V, A>>,
    suffix: Digit<V, A>,
}

impl<V, A> Measurable<V, A> for DeepTree<V, A>
    where V: Measure<A> + Debug,
          A: Measurable<V, A> + Clone + Debug
{
    fn measure(&self) -> V {
        self.m.clone()
    }
}

impl<V, A> Serialize for DeepTree<V, A>
    where A: Debug + Clone + Measurable<V, A>,
          V: Measure<A> + Debug
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{:?}", self.prefix))?;
        serializer.serialize_str(&format!("{:?}", self.middle))?;
        serializer.serialize_str(&format!("{:?}", self.prefix))
        //    	match *self {
        //    		Digit::Zero(_) => panic!(""),
        //    		Digit::One(ref a) => serializer.serialize_str(&format!("{:?}\n",a)),
        //    		Digit::Two(ref a,ref b) => serializer.serialize_str(&format!("{:?},{:?}",a,b)),
        //    		Digit::Three(ref a,ref b,ref c) => serializer.serialize_str(&format!("{:?},{:?},{:?}",a,b,c)),
        //    		Digit::Four(ref a,ref b,ref c,ref d) => serializer.serialize_str(&format!("{:?},{:?},{:?},{:?}",a,b,c,d)),
        //    	}
    }
}

// impl<V:,A:Debug+Clone+Measurable<V,A>> Serialize for DeepTree<V,A> {
//    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//        where S: Serializer
//    {
//    	serializer.serialize_str(&format!("prefix:{:?}---------------suffix:{:?}",self.prefix,self.suffix))?;
//    	serializer.serialize_str(&format!("{:?}",self.middle))?;
//    	Ok(())
//    	}
//    }


// impl<V: Debug, A: Debug + Clone> Display for DeepTree<V, A>
//    where A: Clone + Debug + Measurable<V, A>,
//          V: Measure<A> + Debug
//
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        write!(fmt,
//               "Deep Tree: prefix: {} suffix: {}\n{}\n",
//               self.prefix,
//               self.suffix,
//               self.middle.as_ref())
//    }
//



impl<V, A> Add for DeepTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    type Output = DeepTree<V, A>;
    fn add(self, deep: DeepTree<V, A>) -> Self {
        let m = self.m;
        DeepTree {
            m: m.clone() + deep.m,
            prefix: self.prefix,
            middle: Box::new(unimplemented!()),
            suffix: deep.suffix,
        }
    }
}

impl<V, A> PossiblyEmpty for DeepTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    fn is_empty(&self) -> bool {
        self.measure() > V::default() //FIXME replace this with Monoid::id()
    }
}

impl<V, A> Iterator for DeepTree<V, A>
    where V: Measure<A> + Debug,
          A: Measurable<V, A> + Clone + Debug
{
    type Item = A;
    fn next(&mut self) -> Option<A> {
        match self.prefix.clone() {
            Zero(_) => None,
            One(a) => {
                self.m = V::default();;
                unimplemented!()
            }
            Two(a, b) => {
                self.prefix = Digit::One(b);
                self.m = self.suffix.measure();
                Some(a)
            }
            Three(a, b, c) => unimplemented!(),
            Four(a, b, c, d) => unimplemented!(),
        }
    }
}


impl<V, A> DeepTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    pub fn to_pieces(self) -> (Digit<V, A>, Box<FingerTree<V, A>>, Digit<V, A>) {
        (self.prefix, self.middle, self.suffix)
    }

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
    pub fn new(prefix: Digit<V, A>,
               middle: FingerTree<V, A>,
               suffix: Digit<V, A>)
               -> DeepTree<V, A> {
        DeepTree {
            m: prefix.measure() + middle.measure() + suffix.measure(),
            prefix: prefix,
            middle: Box::new(middle),
            suffix: suffix,
        }
    }

    pub fn from_two(a: A, b: A) -> Self {
        DeepTree {
            m: a.measure() + b.measure(),
            prefix: Digit::One(a),
            middle: Box::new(FingerTree::Empty),
            suffix: Digit::One(b),
        }
    }

    pub fn from_three(a: A, b: A, c: A) -> Self {
        DeepTree {
            m: a.measure() + b.measure() + c.measure(),
            prefix: Digit::Two(a, b),
            middle: Box::new(FingerTree::Empty),
            suffix: Digit::One(c),
        }
    }

    pub fn from_four(a: A, b: A, c: A, d: A) -> Self {
        DeepTree {
            m: a.measure() + b.measure() + c.measure() + d.measure(),
            prefix: Digit::Two(a, b),
            middle: Box::new(FingerTree::Empty),
            suffix: Digit::Two(c, d),
        }
    }

    //    fn add_digits_0<M>(m: M,
    //                               m1: FingerTree<V, A>,
    //                               s1: Digit<A>,
    //                               p2: Digit<A>,
    //                               m2: FingerTree<V, A>)
    //                               -> FingerTree<V, A>
    //        where M: Measurable<V, A>
    //    {
    //        let mk = m.to_tree();
    //        match s1 {
    //            Digit::One(a) => unimplemented!(),
    //            Digit::Two(a, b) => unimplemented!(),
    //            Digit::Three(a, b, c) => unimplemented!(),
    //            Digit::Four(a, b, c, d) => unimplemented!(),
    //
    //        }
    //    }
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
    where V: Measure<A> + Debug,
          A: Measurable<V, A> + Clone + Debug
{
    fn len(&self) -> usize {
        //    	let mid_size = middle.fold_left((acc, n) -> acc + n.length(), 0);
        //        refix.length() + mid_size + suffix.length();
        unimplemented!()
    }

    fn pretty<'b, D>(&'b self, allocator: &'b D) -> DocBuilder<'b, D>
        where D: DocAllocator<'b>
    {
        unimplemented!()
    }


    /// Appends one finger tree to another
    fn append(self, other: Self) -> Self {
        unimplemented!()
    }

    /// Adds the given element to this tree as the first element.
    fn cons(self, t: A) -> FingerTree<V, A> {
        let measure = self.measure();
        match (self.prefix, self.suffix) {
            (Zero(_), _) | (_, Zero(_)) => panic!("impossible"),
            (One(a), One(b)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + t.measure(),
                    prefix: Digit::Two(t, a),
                    middle: self.middle,
                    suffix: Digit::One(b),
                })
            }
            (Two(a, b), One(c)) |
            (One(a), Two(b, c)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + t.measure(),
                    prefix: Digit::Two(t, a),
                    middle: self.middle,
                    suffix: Digit::Two(b, c),
                })
            }
            (Two(a, b), Two(c, d)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + t.measure(),
                    prefix: Digit::Two(t, a),
                    middle: Box::new(self.middle.cons(b)),
                    suffix: Digit::Two(c, d),
                })
            }
            (Two(a, b), Four(c, d, e, f)) |
            (Four(a, b, c, d), Two(e, f)) => unimplemented!(),
            (Three(a, b, c), One(d)) |
            (One(a), Three(b, c, d)) => unimplemented!(),
            (Four(a, b, c, d), One(e)) |
            (One(a), Four(b, c, d, e)) => unimplemented!(),
            (Three(a, b, c), Two(d, e)) |
            (Two(a, b), Three(c, d, e)) => unimplemented!(),
            (Three(a, b, c), Three(d, e, f)) => unimplemented!(),
            (Three(a, b, c), Four(d, e, f, g)) |
            (Four(a, b, c, d), Three(e, f, g)) => unimplemented!(),
            (Four(a, b, c, d), Four(e, f, g, h)) => unimplemented!(),

        }
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
    fn head(self) -> (Option<A>, FingerTree<V, A>) {
        match (self.prefix, self.middle, self.suffix) {
            // If there is nothing in the prefix, it should mean the rest of the tree is empty, assuming good housekeeping
            (Zero(_), _, _) => (None, FingerTree::Empty), 
            (One(a), middle, suffix) => {
                // println!("one prefix {:?}",a);

                match middle.head() {
                    (None, _) => {
                        // If there is no middle promote the suffix to a prefix?
                        // println!("no middle, suffix: {:?}", suffix);
                        (Some(a), suffix.to_tree())
                    }
                    (Some(b), middle) => {
                        // if there is a middle, put b as the prefix call head() on middle and return head,
                        // then call head on the remained and put the head in the prefix
                        // println!("some middle {:?}",middle);
                        let measure = b.measure() + middle.measure() + suffix.measure();
                        (Some(a),
                         FingerTree::Deep(DeepTree {
                             m: measure,
                             prefix: Digit::One(b),
                             middle: Box::new(middle),
                             suffix: suffix,
                         }))
                    }
                }
            }

            (Two(a, b), middle, suffix) => {
                // println!("two prefix {:?} {:?}",a,b);

                (Some(a),
                 DeepTree {
                         m: b.measure() + middle.measure() + suffix.measure(),
                         prefix: Digit::One(b),
                         middle: Box::new(*middle),
                         suffix: suffix,
                     }
                     .to_tree())
            }

            (Three(a, b, c), middle, suffix) => {
                unimplemented!();
                let prefix = Digit::Two(b, c);
                (Some(a),
                 FingerTree::Deep(DeepTree {
                     m: prefix.measure() + self.suffix.measure(),
                     prefix: prefix,
                     middle: Box::new(FingerTree::Empty),
                     suffix: self.suffix,
                 }))
            }
            (Four(a, b, c, d), middle, suffix) => {
                unimplemented!();
                let prefix = Digit::Three(b, c, d);
                (Some(a),
                 FingerTree::Deep(DeepTree {
                     m: prefix.measure() + self.middle.measure() + self.suffix.measure(),
                     prefix: prefix,
                     middle: Box::new(FingerTree::Empty),
                     suffix: self.suffix,
                 }))
            }
        }
    }

    fn lookup(&self, o: &Fn(V) -> usize, i: usize) -> (usize, Option<A>) {
        let spr = o(self.prefix.measure());
        if i < spr {
            return self.prefix.lookup(o, i);
        } else {
            let spm = spr + o(self.middle.measure());
            if i < spm {
                let p = self.middle.lookup(o, i - spr);
                return (i, p.1);
            } else {
                return self.suffix.lookup(o, i - spm);
            }
        }

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

    /// Adds the given element to this tree as the last element
    fn snoc(self, last: A) -> FingerTree<V, A> {
        let measure = self.measure();
        match (self.prefix, self.suffix) {
            (Zero(_), _) | (_, Zero(_)) => panic!("impossible"),
            (One(a), One(b)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + last.measure(),
                    prefix: Digit::One(a),
                    middle: self.middle,
                    suffix: Digit::Two(b,last),
                })
            }
            (Two(a, b), One(c)) |
            (One(a), Two(b, c)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + last.measure(),
                    prefix: Digit::Two(a,b),
                    middle: self.middle,
                    suffix: Digit::Two(c,last),
                })
            }
            (Two(a, b), Two(c, d)) => {
                FingerTree::Deep(DeepTree {
                    m: measure + last.measure(),
                    prefix: Digit::Two(a,b),
                    middle: Box::new(self.middle.snoc(c)),
                    suffix: Digit::Two(d,last),
                })
            }
            (Two(a, b), Four(c, d, e, f)) |
            (Four(a, b, c, d), Two(e, f)) => unimplemented!(),
            (Three(a, b, c), One(d)) |
            (One(a), Three(b, c, d)) => unimplemented!(),
            (Four(a, b, c, d), One(e)) |
            (One(a), Four(b, c, d, e)) => unimplemented!(),
            (Three(a, b, c), Two(d, e)) |
            (Two(a, b), Three(c, d, e)) => unimplemented!(),
            (Three(a, b, c), Three(d, e, f)) => unimplemented!(),
            (Three(a, b, c), Four(d, e, f, g)) |
            (Four(a, b, c, d), Three(e, f, g)) => unimplemented!(),
            (Four(a, b, c, d), Four(e, f, g, h)) => unimplemented!(),

        }
    }

    /// splits this tree into a pair of subtrees at the point where the given predicate,
    /// based on the measure, changes from false to true.
    fn split(self, f: &Fn(V) -> bool) -> (Self, Option<Self>)
        where Self: Sized
    {
        unimplemented!()
    }

    fn to_tree(self) -> FingerTree<V, A> {
        FingerTree::Deep(self)
    }
}
