/// Extensionality Axiom
///
/// For all sets `A` and `B`, if `A` and `B` have the same elements, then `A = B`.
pub trait Extensionality<T> {
    fn is_equal(&self, other: &T) -> bool;
}

/// Cardinality Axiom
///
/// For every set `A`, there exists a natural number `|A|`, called the cardinality of `A`,
pub trait Cardinality {
    fn cardinality(&self) -> usize;
}

/// Empty Set Axiom
///
/// There exists a set `Ø`, called the empty set, such that for all sets `A`, `Ø ∉ A`.
pub trait EmptySet {
    fn is_empty(&self) -> bool;
}

/// Pair Set Axiom
///
/// For every two sets `A` and `B`, there exists a set `{A, B}`.
pub trait PairSet<T, U> {
    fn new(a: T, b: U) -> Self;
}

/// Union Axiom
///
/// For every set `A` and every set `B`, there exists a set `A ∪ B`, called the union of `A` and `B`,
/// such that `x ∈ A ∪ B` if and only if `x ∈ A` or `x ∈ B`.
pub trait Union<T> {
    fn union(&self, other: &T) -> Self;
}

/// Intersection Axiom
///
/// For every set `A` and every set `B`, there exists a set `A ∩ B`, called the intersection of `A` and `B`,
/// such that `x ∈ A ∩ B` if and only if `x ∈ A` and `x ∈ B`.
pub trait Intersection<T> {
    fn intersection(&self, other: &T) -> Self;
}

/// Difference Axiom
///
/// For every set `A` and every set `B`, there exists a set `A - B`, called the difference of `A` and `B`,
/// such that `x ∈ A - B` if and only if `x ∈ A` and `x ∉ B`.
pub trait Difference<T> {
    fn difference(&self, other: &T) -> Self;
}

/// Complement Axiom
///
/// For every set `A`, there exists a set `A'`, called the complement of `A`,
/// such that `x ∈ A'` if and only if `x ∉ A`.
pub trait Complement {
    fn complement(&self) -> Self;
}

/// Regularity Axiom
///
/// For every set `A`, there exists a set `B` such that `A ∩ B = Ø`.
pub trait Regularity {
    fn is_disjoint(&self, other: &Self) -> bool;
}

/// Power Set Axiom
///
/// For every set `A`, there exists a set `P(A)`, called the power set of `A`,
/// such that `B ∈ P(A)` if and only if `B ⊆ A`.
pub trait PowerSet {
    fn power_set(&self) -> Self;
}

/// Replacement Axiom
///
/// If `f` is a function from a set `A` to a set `B`, and if `C` is the image of `A` under `f`, then `C` is a set.
pub trait Replacement<A, B> {
    fn replace(&self, f: &dyn Fn(A) -> B) -> Self;
}

/// Axiom of Choice
///
/// Given any collection of non-empty sets, it is possible to choose one element from each set.
pub trait Choice<T> {
    fn choose(&self) -> T;
}

/// Axiom of Specification
///
/// The axiom of specification: This axiom states that if a set X and a property P are given, then there exists a set Y consisting of exactly those elements of X that have property P. For example, if X is the set of all natural numbers, and P is the property "even," then Y would be the set of all even natural numbers.
pub trait Specification<T> {
    fn is_in_set(&self, elem: &T) -> bool;
}

impl<T, F> Specification<T> for F
where
    F: Fn(&T) -> bool,
{
    fn is_in_set(&self, elem: &T) -> bool {
        (self)(elem)
    }
}

/// Axiom of Induction
///
/// The axiom of induction: This axiom states that if a property P is true for the number 0 and for all natural numbers, then it is true for all natural numbers. For example, if P is the property "is even," then P is true for 0 and for all natural numbers, so P is true for all natural numbers.
pub trait Induction<T> {
    fn base_case(&self) -> bool;
    fn inductive_step(&self, elem: &T) -> bool;
}
