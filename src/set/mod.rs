mod axioms;
mod domain;
mod finite_set;
mod functions;
pub mod natural;
mod non_determinate_set;
pub mod operations;

/// A set is a collection of elements.
pub trait Set {
    type Element;

    fn contains(&self, e: &Self::Element) -> bool;
    fn insert(&mut self, e: Self::Element);
    fn remove(&mut self, e: &Self::Element);
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn clear(&mut self);
}
