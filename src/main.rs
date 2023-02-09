use crate::set::natural::NaturalNumber;
use crate::set::operations::Add;

mod matrix;
mod numbers;
mod printer;
mod set;

fn main() {
    let n1 = NaturalNumber::from_usize(2);
    let n2 = NaturalNumber::from_usize(3);

    let n3 = n1.add(&n2);
    println!("{n1:?} + {n2:?} = {n3:?}");
    println!("n3 = {}", n3.to_usize());
}
