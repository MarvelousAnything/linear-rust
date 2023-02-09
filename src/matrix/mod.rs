mod traits;
mod operations;
mod space;
mod vector;

use num_traits::Float;
use std::fmt::{Debug, Display};
pub use traits::*;

impl<const R: usize, const C: usize, T: Float + Debug + Display> Matrix<R, C> for [[T; C]; R] {
    type Element = T;

    fn new() -> Self {
        [[T::zero(); C]; R]
    }

    fn get(&self, r: usize, c: usize) -> Self::Element {
        self[r][c]
    }

    fn set(&mut self, r: usize, c: usize, v: Self::Element) {
        self[r][c] = v;
    }

    fn row(&self, r: usize) -> [Self::Element; C] {
        self[r]
    }

    fn col(&self, c: usize) -> [Self::Element; R] {
        let mut col = [T::zero(); R];
        for i in 0..R {
            col[i] = self[i][c];
        }
        col
    }
}
