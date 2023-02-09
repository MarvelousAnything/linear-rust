use crate::set::axioms::Extensionality;
use crate::set::Set;

#[derive(Debug, Clone)]
pub struct FiniteSet<T> {
    elements: Vec<T>,
}

impl<T: PartialEq> Set for FiniteSet<T> {
    type Element = T;

    fn contains(&self, e: &Self::Element) -> bool {
        self.elements.contains(e)
    }

    fn insert(&mut self, e: Self::Element) {
        if !self.contains(&e) {
            self.elements.push(e);
        }
    }

    fn remove(&mut self, e: &Self::Element) {
        if self.contains(e) {
            self.elements.retain(|x| x != e);
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }

    fn clear(&mut self) {
        self.elements.clear();
    }
}

impl<T: PartialEq> Extensionality<FiniteSet<T>> for FiniteSet<T> {
    fn is_equal(&self, other: &FiniteSet<T>) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for e in self.elements.iter() {
            if !other.contains(e) {
                return false;
            }
        }
        true
    }
}
