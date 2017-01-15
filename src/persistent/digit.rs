use kinder::lift::{Monoid, SemiGroup};
use std::ops::{Mul, Add};
use persistent::*;
use std::fmt::{Debug, Formatter};
use std::fmt;
use serde_json::value::ToJson;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use serde::{Serialize, Serializer};

#[derive(Clone,Eq,Ord,PartialEq,PartialOrd,Debug)]
pub enum Digit<V, A: Debug> {
    Zero(V),
    One(A),
    Two(A, A),
    Three(A, A, A),
    Four(A, A, A, A),
}

impl<V, A: Debug> Serialize for Digit<V, A> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            Digit::Zero(_) => panic!(""),
            Digit::One(ref a) => serializer.serialize_str(&format!("{:?}\n", a)),
            Digit::Two(ref a, ref b) => serializer.serialize_str(&format!("{:?},{:?}", a, b)),
            Digit::Three(ref a, ref b, ref c) => {
                serializer.serialize_str(&format!("{:?},{:?},{:?}", a, b, c))
            }
            Digit::Four(ref a, ref b, ref c, ref d) => {
                serializer.serialize_str(&format!("{:?},{:?},{:?},{:?}", a, b, c, d))
            }
        }
    }
}



// impl<V, A> ToJson for Digit<V, A>
//    where A: Display,
//          V: Display
//
//    fn to_json(&self) -> Json {
//        match *self.clone() {
//            Digit::Zero(ref v) => serde_json::to_string(format!("No Prefix with measure {}", v)),
//            One(ref a) => Json::String(format!("{}", a)),
//            Two(ref a, ref b) => Json::String(format!("{},{}", a, b)),
//            Three(ref a, ref b, ref c) => Json::String(format!("{},{},{}", a, b, c)),
//            Four(ref a, ref b, ref c, ref d) => Json::String(format!("{},{},{},{}", a, b, c, d)),
//        }
//    }
//

// impl<V: Debug, A: Debug + Clone> Display for Digit<V, A>
//    where A: Clone + Debug + Measurable<V, A>,
//          V: Measure<A> + Debug
//
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        write!(fmt, "Deep Tree: {:?}\n", self)
//    }
//
// impl<A> Mul<A> for Digit<A> {
//
//

//  private final class DigitMeasure[V, A](m: Measure[A, V]) extends Measure[Digit[V, A], V] {
//    def zero: V = m.zero
//    def apply(n: Digit[V, A]): V = n.measure
//
//    def |+|(a: V, b: V): V = m |+|(a, b)
//    override def |+|(a: V, b: V, c: V): V = m |+|(a, b, c)
//  }
impl<V, A> Measure<V> for Digit<V, A>
    where V: Measurable<Digit<V, A>, V> + Default + Clone + PartialOrd,
          A: Default + Clone + Measurable<Digit<V, A>, A> + PartialOrd + Debug,
          Digit<V, A>: Mul<Output = Digit<V, A>> + Measurable<Digit<V, A>, V>,
          Digit<V, A>: Measure<A>
{
}

// impl<A> Measurable<Digit<A>, V> for Digit<A> {}
use self::Digit::*;

impl<V, A> Monoid for Digit<V, A>
    where A: Default + Debug
{
    fn id() -> A {
        A::default()
    }
}

impl<V, A: Debug> Add for Digit<V, A> {
    type Output = Digit<V, A>;
    fn add(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl<V, A> Default for Digit<V, A>
    where A: Default + Debug
{
    fn default() -> Self {
        Digit::One(A::default())
    }
}

impl<V, A: Debug> SemiGroup for Digit<V, A> {
    type A = A;
    fn add(&self, other: &A) -> A {
        unimplemented!()
    }
}
impl<V, A: Debug> Digit<V, A> {
    pub fn fold_map<M, F>(&self, f: F) -> M
        where M: Add<Output = M>,
              F: Fn(&A) -> M
    {
        match *self {
            Zero(_) => unimplemented!(),
            One(ref a) => f(a),
            Two(ref a, ref b) => f(a) + f(b),
            Three(ref a, ref b, ref c) => f(a) + f(b) + f(c),
            Four(ref a, ref b, ref c, ref d) => f(a) + f(b) + f(c) + f(d),
        }
    }

    pub fn new(a: A) -> Self {
        Digit::One(a)
    }
}

impl<V, A: Debug> Measurable<V, A> for Digit<V, A>
    where A: Measurable<V, A> + Clone,
          V: Measure<A>
{
    fn measure(&self) -> V {
        self.fold_map(|d| Measurable::measure(d))
    }
}

impl<V, A: Debug> IntoIterator for Digit<V, A>
    where V: Measure<A>,
          A: Clone,
          A: Measurable<V, A>
{
    type Item = A;
    type IntoIter = IntoDigitIter<V, A>;
    fn into_iter(self) -> IntoDigitIter<V, A> {
        IntoDigitIter(self)
    }
}

pub struct IntoDigitIter<V, A: Debug>(Digit<V, A>);

impl<V, A: Debug> Iterator for IntoDigitIter<V, A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        unimplemented!()
    }
}


impl<V, A> TreeLike<V, A> for Digit<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    fn len(&self) -> usize {unimplemented!()}
    fn pretty<'b, D>(&'b self, allocator: &'b D) -> DocBuilder<'b, D> where D: DocAllocator<'b> {unimplemented!()}

    fn to_tree(self) -> FingerTree<V, A> {
        match self {
            Digit::Zero(_) => FingerTree::Empty,
            Digit::One(a) => FingerTree::Single(a),
            Digit::Two(a, b) => FingerTree::Deep(DeepTree::from_two(a, b)),
            Digit::Three(a, b, c) => FingerTree::Deep(DeepTree::from_three(a, b, c)),
            Digit::Four(a, b, c, d) => FingerTree::Deep(DeepTree::from_four(a, b, c, d)),
        }
        // 		FingerTree::Deep(DeepTree {
        //                    measure: self.measure(),
        //                    prefix: Digit::One(self.0),
        //                    middle:Box::new(FingerTree::Empty),
        //                    suffix:Digit::One(self.1)
        //                })
    }



    /// Appends one finger tree to another
    fn append(self, other: Self) -> Self {
        unimplemented!()
    }

    /// Adds the given element to this tree as the first element.
    fn cons(self, t: A) -> FingerTree<V, A> {
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
    fn head(self) -> (Option<A>, FingerTree<V, A>) {
        match self {
            Zero(a) => panic!("impossible"),
            One(a) => (Some(a), FingerTree::Empty),
            Two(a, b) => (Some(a), FingerTree::Single(b)),
            Three(a, b, c) => (Some(a), Digit::Two(b, c).to_tree()),
            Four(a, b, c, d) => (Some(a), Digit::Three(b, c, d).to_tree()),
        }
    }

    fn lookup(&self, o: &Fn(V) -> usize, i: usize) -> (usize, Option<A>) {
        unimplemented!()
    }

    //    ///Maps the given function across this tree, measuring with the given Measured instance.
    //    /// fn map<B, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> B{unimplemented!()}
    //    fn measure(self) -> (V, Self){unimplemented!()}

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
    fn snoc(self, t: A) -> FingerTree<V, A> {
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
