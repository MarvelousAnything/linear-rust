use crate::set::axioms::{EmptySet, Union};
use std::cmp::Ordering;

/// A trait that defines the basic operations for a non-determinant set.
pub trait NonDeterminantSet: Sized {
    fn membership_rule(&self) -> Box<dyn Fn(&Self) -> bool>;
}
