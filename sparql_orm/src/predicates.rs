use crate::{triple_pattern::TriplePattern, sparql_var::SPQLVar};

///
/// A marker trait to indicate types that 
/// represent a valid set of predicates
///
pub trait PredicateSet {}

pub trait Predicate {}

impl<T: Predicate, const N: usize> PredicateSet for [T; N] {}

impl<SU, PR, VR> Predicate for TriplePattern<SU, PR, VR> where 
SU: SPQLVar, PR: SPQLVar, VR: SPQLVar
{

}



