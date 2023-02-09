pub trait PeanoAxioms {
    /// Zero is a natural number
    fn zero() -> Self;

    /// The successor of a natural number is a natural number
    fn successor(&self) -> Self;

    /// Two natural numbers are equal if their successors are equal
    fn eq(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NaturalNumber {
    Zero,
    Successor(Box<NaturalNumber>),
}

impl PeanoAxioms for NaturalNumber {
    fn zero() -> Self {
        NaturalNumber::Zero
    }

    fn successor(&self) -> Self {
        NaturalNumber::Successor(Box::new(self.clone()))
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NaturalNumber::Zero, NaturalNumber::Zero) => true,
            (NaturalNumber::Successor(n), NaturalNumber::Successor(m)) => n.eq(m),
            _ => false,
        }
    }
}

impl NaturalNumber {
    pub fn to_usize(&self) -> usize {
        match self {
            NaturalNumber::Zero => 0,
            NaturalNumber::Successor(n) => n.to_usize() + 1,
        }
    }

    pub fn from_usize(n: usize) -> Self {
        match n {
            0 => NaturalNumber::Zero,
            _ => NaturalNumber::Successor(Box::new(NaturalNumber::from_usize(n - 1))),
        }
    }
}
