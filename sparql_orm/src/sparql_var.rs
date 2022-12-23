//!
//! WIP, ideal end state is to have
//!
//!
//! A marker trait that indicates that a type is a valid
//! variable, that is usable in a triple or any other
//! place where a valid variable or binding can be used

pub trait SPQLVar {}

use crate::identifier::*;

pub struct Literal<T: Identifier> {
    v: T,
}
pub struct Variable<T: Identifier> {
    v: T,
}

impl<T> SPQLVar for Literal<T> where T: Identifier {}
impl<T> SPQLVar for Variable<T> where T: Identifier {}
