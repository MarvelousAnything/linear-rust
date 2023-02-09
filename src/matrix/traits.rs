use num_traits::{Float, One, Zero};
use std::fmt::{Debug, Display};

pub trait Matrix<const R: usize, const C: usize>: Debug + Sized + Clone {
    type Element: Float + Debug + Display + ?Sized;
    fn new() -> Self;
    fn row_len(&self) -> usize {
        R
    }
    fn col_len(&self) -> usize {
        C
    }
    fn get(&self, r: usize, c: usize) -> Self::Element;
    fn set(&mut self, r: usize, c: usize, v: Self::Element);
    fn row(&self, r: usize) -> [Self::Element; C];
    fn col(&self, c: usize) -> [Self::Element; R];
    fn as_list(&self) -> [[Self::Element; C]; R] {
        let mut list = [[Self::Element::zero(); C]; R];
        for r in 0..R {
            let row = self.row(r);
            list[r] = row;
        }
        list
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        let matrix = self.as_list();
        for row in matrix.iter() {
            for elem in row.iter() {
                s.push_str(&format!("{elem:8.2}"));
            }
            s.push('\n');
        }
        s
    }
}

pub trait Identity<const N: usize>
where
    Self: Matrix<N, N>,
{
    fn identity() -> Self;
}

pub trait Traceable<const N: usize>
where
    Self: Matrix<N, N>,
{
    fn trace(&self) -> Self::Element;
}

pub trait Transposable<const R: usize, const C: usize>
where
    Self: Matrix<R, C>,
{
    fn transpose(&self) -> Self;
}

pub trait Determinable<const N: usize>
where
    Self: Matrix<N, N>,
{
    fn determinant(&self) -> Self::Element;
}

pub trait Cofactor<const N: usize>
where
    Self: Matrix<N, N>,
{
    fn cofactor(&self, r: usize, c: usize) -> Self::Element;
}

pub trait LUDecomposable<const N: usize>
where
    Self: Matrix<N, N>,
{
    fn lu_decompose(&self) -> (Self, Self);
}

impl<const N: usize, M: Matrix<N, N>> Identity<N> for M {
    fn identity() -> Self {
        let mut m = Self::new();
        for i in 0..N {
            m.set(i, i, Self::Element::one());
        }
        m
    }
}

impl<const N: usize, M: Matrix<N, N>> Traceable<N> for M {
    fn trace(&self) -> Self::Element {
        let mut trace = Self::Element::zero();
        for i in 0..N {
            trace = trace + self.get(i, i);
        }
        trace
    }
}

impl<const N: usize, M: Matrix<N, N>> Cofactor<N> for M {
    fn cofactor(&self, r: usize, c: usize) -> Self::Element {
        let mut sub_matrix = Self::new();
        let mut i = 0;
        let mut j = 0;

        for row in 0..N {
            if row == r {
                continue;
            }
            for col in 0..N {
                if col == c {
                    continue;
                }
                sub_matrix.set(i, j, self.get(row, col));
                j += 1;
                if j == N - 1 {
                    break;
                }
            }
            i += 1;
            j = 0;
        }

        let sub_det = sub_matrix.determinant();
        let factor = if (r + c) % 2 == 0 {
            Self::Element::one()
        } else {
            -Self::Element::one()
        };
        factor * sub_det
    }
}

impl<const N: usize, M: Matrix<N, N> + LUDecomposable<N> + Traceable<N>> Determinable<N> for M {
    fn determinant(&self) -> Self::Element {
        let (l, u) = self.lu_decompose();
        let mut det: Self::Element = Self::Element::one();
        for i in 0..N {
            det = det * u.get(i, i);
        }
        det * l.trace().signum()
    }
}

impl<const N: usize, M: Matrix<N, N>> LUDecomposable<N> for M {
    fn lu_decompose(&self) -> (Self, Self) {
        let mut l = Self::identity();
        let mut u = self.clone();

        for j in 0..N {
            for i in j + 1..N {
                let mut lij: Self::Element = u.get(i, j) / u.get(j, j);
                if lij.is_nan() {
                    lij = Self::Element::zero();
                }
                for k in j..N {
                    u.set(i, k, u.get(i, k) - lij * u.get(j, k));
                }
                l.set(i, j, lij);
            }
        }

        (l, u)
    }
}
