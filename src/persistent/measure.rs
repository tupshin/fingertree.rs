use kinder::lift::Monoid;

use persistent::*;

pub trait Measurable<V, A>
    where V: Measure<A>,
          A: Measurable<V, A> + Clone
{
    fn measure(&self) -> V;
    fn to_tree(self) -> FingerTree<V, A>;
}

// pub struct Measured<V>(V);
//
// impl<V> Measurable<V> for Measured<V> where V:Monoid{
// 	fn measure(&self) -> V {
// 		self.0
// 	}
//



impl<'a, V, A: 'a> Measurable<V, A> for FingerTree<V, A>
    where A: Measurable<V, A> + Clone,
          V: Measure<A>
{
    fn to_tree(self) -> FingerTree<V, A> {
        unimplemented!()
    }
    fn measure(&self) -> V {
        match self {
            &FingerTree::Empty => V::default(),
            &FingerTree::Single(ref x) => x.measure(),
            &FingerTree::Deep(ref d) => d.measure().clone(),
        }
    }
}
