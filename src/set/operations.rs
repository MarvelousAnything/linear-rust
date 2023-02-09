use crate::set::natural::NaturalNumber;

pub trait Add<RHS = Self> {
    type Output;

    fn add(&self, rhs: &RHS) -> Self::Output;
}

impl Add for NaturalNumber {
    type Output = NaturalNumber;

    fn add(&self, rhs: &NaturalNumber) -> Self::Output {
        match self {
            NaturalNumber::Zero => rhs.clone(),
            NaturalNumber::Successor(n) => NaturalNumber::Successor(Box::new(n.add(rhs))),
        }
    }
}
