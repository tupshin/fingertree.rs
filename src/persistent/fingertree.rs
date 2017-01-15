use persistent::*;
use kinder::lift::Monoid;
use std::ops::Mul;
use self::FingerTree::*;
use std::ops::{Deref, Add};
use std::fmt::{Debug, Display};
use std::fmt;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use std::ops::Range;

#[derive(Ord,PartialOrd,PartialEq,Eq,Debug,Clone,Serialize)]
pub enum FingerTree<V, A>
    where A: Clone + Debug + Measurable<V, A>,
          V: Measure<A> + Debug
{
    Empty,
    Single(A),
    Deep(DeepTree<V, A>),
}


// impl<V: Debug, A: Debug + Clone> Display for FingerTree<V, A>
//    where A: Clone + Debug + Measurable<V, A>,
//          V: Measure<A> + Debug
//
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        match self {
//            &Empty => write!(fmt, "Empty Tree"),
//            &Single(ref a) => write!(fmt, "Single Node Tree: {:?}", a),
//            &Deep(ref d) => write!(fmt, "{}\n", d),
//        }
//    }
//
//

impl<V, A> Add for FingerTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    type Output = FingerTree<V, A>;

    fn add(self, other: Self) -> FingerTree<V, A> {
        match (self, other) {
            (Empty, Empty) => FingerTree::Empty,
            (Single(s), Empty) |
            (Empty, Single(s)) => FingerTree::Single(s),
            (Single(s1), Single(s2)) => {
                FingerTree::Deep(DeepTree::new(Digit::new(s1), FingerTree::Empty, Digit::new(s2)))
            }
            (Deep(d), Empty) | (Empty, Deep(d)) => FingerTree::Deep(d),
            (Deep(d), Single(s)) |
            (Single(s), Deep(d)) => d.snoc(s),
            (Deep(d1), Deep(d2)) => FingerTree::Deep(d1 + d2),
        }
    }
}

impl<V, A> From<A> for FingerTree<V, A>
    where A: Clone + Debug + Measurable<V, A>,
          V: Measure<A> + Debug
{
    fn from(a: A) -> Self {
        FingerTree::Single(a)
    }
}

// fn v_to_size(v:V) -> usize {
// 	v
//

impl<V, A> TreeLike<V, A> for FingerTree<V, A>
    where V: Measure<A> + Debug,
          A: Clone + Measurable<V, A> + Debug
{
    fn pretty<'b, D>(&'b self, allocator: &'b D) -> DocBuilder<'b, D>
        where D: DocAllocator<'b>
    {
        let forest = self;
        //        let mut doc = allocator.nil();
        //        let mut i = 0;
        //        let k = forest.len() - 1;
        //        loop {
        //            if i < k {
        //                doc = doc.append(forest.lookup( &Fn(V) -> usize,i)
        //                    .pretty(allocator)
        //                    .append(allocator.text(","))
        //                    .append(allocator.newline()));
        //            } else if i == k {
        //                doc = doc.append(forest[i].pretty(allocator));
        //                break;
        //            }
        //            i += 1;
        //        }
        //        doc
        unimplemented!()
    }

    /// Appends one finger tree to another
    fn append(self, other: Self) -> Self {
        match (self, other) {
            (Empty, t) | (t, Empty) => t,
            (Single(a1), Single(a2)) => {
                FingerTree::Deep(DeepTree::new(Digit::new(a1), FingerTree::Empty, Digit::new(a2)))
            }
            (Single(a), Deep(d)) |
            (Deep(d), Single(a)) => d.snoc(a),
            (Deep(d1), Deep(d2)) => d1.append(d2).into(),
        }
    }

    fn len(&self) -> usize {
        match *self {
            Empty => 0,
            Single(_) => 1,
            Deep(ref d) => d.len(),
        }
    }

    /// Adds the given element to this tree as the first element.
    fn cons(self, first: A) -> FingerTree<V, A> {
        match self {
            Empty => FingerTree::Single(first),
            Single(a) => Digit::Two(first, a).to_tree(),
            Deep(d) => d.cons(first),
        }
    }

    /// Folds the tree to the left with the given function and the given initial element
    fn fold_left<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(B) -> F2,
              F2: Fn(A) -> B
    {
        match self {
            Empty => b,
            Single(a) => f(b)(a),
            Deep(d) => d.fold_left(f, b),
        }
    }

    /// Folds the tree to the right with the given function and the given initial element
    fn fold_right<B, F1, F2>(self, f: F1, b: B) -> B
        where F1: Fn(A) -> F2,
              F2: Fn(B) -> B
    {
        match self {
            Empty => b,
            Single(a) => f(a)(b),
            Deep(d) => d.fold_right(f, b),
        }
    }

    /// The first element of this tree
    fn head(self) -> (Option<A>, Self) {
        match self {
            Empty => (None, self),
            Single(s) => (Some(s), FingerTree::Empty),
            Deep(d) => d.head(),

        }
    }

    fn lookup(&self, o: &Fn(V) -> usize, i: usize) -> (usize, Option<A>) {
        match *self {
            Empty => (0, None),
            Single(ref s) => (i, Some(s.clone())),
            Deep(ref d) => d.lookup(o, i),
        }
    }

    /// Folds the tree to the left with the given function
    fn reduce_left<F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(A, A) -> A
    {
        match self {
            Empty => None,
            Single(a) => Some(a),
            Deep(d) => d.reduce_left(f),
        }
    }

    /// Folds the tree to the right with the given function
    fn reduce_right<B, F1, F2>(self, f: F1) -> Option<A>
        where F1: Fn(A, F2),
              F2: Fn(B, B) -> A
    {
        match self {
            Empty => None,
            Single(a) => Some(a),
            Deep(d) => d.reduce_right(f),
        }
    }

    /// Adds the given element to this tree as the last element
    fn snoc(self, last: A) -> FingerTree<V, A> {
        match self {
            Empty => FingerTree::Single(last),
            Single(a) => Digit::Two(a, last).to_tree(),
            Deep(d) => d.snoc(last),
        }
    }

    /// splits this tree into a pair of subtrees at the point where the given predicate,
    /// based on the measure, changes from false to true.
    fn split(self, f: &Fn(V) -> bool) -> (Self, Option<Self>)
        where Self: Sized
    {
        match self {
            Empty | Single(_) => (self, None),
            Deep(d) => {
                let (a, b) = d.split(f);
                (FingerTree::Deep(a),
                 Some((match b {
                     Some(b) => FingerTree::Deep(b),
                     None => FingerTree::Empty,
                 })))
            }
        }
    }

    fn to_tree(self) -> FingerTree<V, A> {
        self
    }
}

impl<V, A> IntoIterator for FingerTree<V, A>
    where V: Measure<A> + Debug,

          A: Clone + Measurable<V, A> + Debug
{
    type Item = A;
    type IntoIter = IntoIter<V, A>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[derive(Debug)]
pub struct IntoIter<V, A>(FingerTree<V, A>)
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug;

impl<V, A> Iterator for IntoIter<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    type Item = A;
    fn next(&mut self) -> Option<A> {
        match self.0.clone() {
            Empty => None,
            Single(a) => {
                self.0 = FingerTree::Empty;
                Some(a.clone())
            }
            Deep(d) => {
                let (head, tail) = d.head();
                self.0 = tail;
                head
            }
        }
        // let z = self.0;
    }
}

// impl<V, A> Iterator for FingerTree<V, A>
//    where V: Monoid + Default + Clone,
//
//          A: Clone + Measurable<V>
//
//    type Item = A;
//    fn next(&mut self) -> Option<A> {
//        match self {
//            &mut Empty => None,
//            &mut Single(a) => {
//                self = &mut FingerTree::Empty;
//                Some(a)
//            }
//            &mut Deep(ref mut d) => d.next(),
//        }
//    }
//

impl<V, A> PossiblyEmpty for FingerTree<V, A>
    where A: Clone + Measurable<V, A> + Debug,
          V: Measure<A> + Debug
{
    fn is_empty(&self) -> bool {
        match *self {
            FingerTree::Empty => true,
            FingerTree::Single(_) => false,
            FingerTree::Deep(ref d) => d.is_empty(),
        }
    }
}



impl<'a, V, A: 'a + Clone> FingerTree<V, A>
    where V: Measure<A> + Debug,
          A: Measurable<V, A> + Debug
{
    pub fn is_empty(&self) -> bool {
        match *self {
            FingerTree::Empty => true,
            FingerTree::Single(_) => false,
            FingerTree::Deep(ref d) => d.is_empty(),
        }
    }
}

pub trait PossiblyEmpty {
    fn is_empty(&self) -> bool;
}


impl<'a, A: 'a> PossiblyEmpty for [A] {
    fn is_empty(&self) -> bool {
        if self.len() == 0 { true } else { false }
    }
}

impl<'a, A: 'a> PossiblyEmpty for Vec<A> {
    fn is_empty(&self) -> bool {
        if self.len() == 0 { true } else { false }
    }
}


impl<V, A> From<DeepTree<V, A>> for FingerTree<V, A>
    where A: Clone + Debug + Measurable<V, A>,
          V: Measure<A> + Debug
{
    fn from(d: DeepTree<V, A>) -> Self {
        FingerTree::Deep(d)
    }
}

#[cfg(test)]
mod tests {
    use persistent::fingertree::*;
    #[test]
    fn char_cons() {
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

        let items: Vec<char> = t.into_iter().collect();
        assert_eq!(items,
                   vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n']);
    }

    #[test]
    fn i32_cons() {
        let t: FingerTree<isize, i32> = FingerTree::Empty;
        let t = t.cons(14)
            .cons(13)
            .cons(12)
            .cons(11)
            .cons(10)
            .cons(9)
            .cons(8)
            .cons(7)
            .cons(6)
            .cons(5)
            .cons(4)
            .cons(3)
            .cons(2)
            .cons(1);

        let items: Vec<i32> = t.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    #[test]
    fn i32_snoc() {
        let t: FingerTree<isize, i32> = FingerTree::Empty;
        let t = t.cons(1)
            .snoc(2)
            .snoc(3)
            .snoc(4)
            .snoc(5)
            .snoc(6)
            .snoc(7)
            .snoc(8)
            .snoc(9)
            .snoc(10)
            .snoc(11)
            .snoc(12)
            .snoc(13)
            .snoc(14);

        let items: Vec<i32> = t.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    #[test]
    fn i32_medium_snoc() {
        let mut t: FingerTree<isize, i32> = FingerTree::Empty;
        let snoc_items: Vec<i32> = (1..1000).collect();
        for i in snoc_items.clone() {
            t = t.snoc(i);
        }
        let popped_items: Vec<i32> = t.into_iter().collect();
        assert_eq!(snoc_items, popped_items)
    }

    #[test]
    fn i32_big_snoc() {
        let mut t: FingerTree<isize, i32> = FingerTree::Empty;
        let snoc_items: Vec<i32> = (1..10000).collect();
        for i in snoc_items.clone() {
            t = t.snoc(i);
        }
        let popped_items: Vec<i32> = t.into_iter().collect();
        assert_eq!(snoc_items, popped_items)
    }


    #[test]
    fn char_snoc() {
        let t: FingerTree<isize, char> = FingerTree::Empty;
        let t = t.snoc('a')
            .snoc('b')
            .snoc('c')
            .snoc('d')
            .snoc('e')
            .snoc('f')
            .snoc('g')
            .snoc('h')
            .snoc('i')
            .snoc('j')
            .snoc('k')
            .snoc('l')
            .snoc('m')
            .snoc('n');

        let items: Vec<char> = t.into_iter().collect();
        assert_eq!(items,
                   vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n']);
    }
}
