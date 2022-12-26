use crate::query_build::QueryFragment;
use crate::{sparql_var::SPQLVar, triple_pattern::TriplePattern};
///
/// A marker trait to indicate types that
/// represent a valid set of predicates
///
pub trait PredicateSet: QueryFragment {}

pub trait Predicate: QueryFragment {}

/// A basic collection type to represent sets of predicates,
/// Currently this only matches a single predicate type,
/// so this won't support heterogeneous predicates
impl<T: Predicate, const N: usize> PredicateSet for [T; N] {}

impl<SU, PR, VR> Predicate for TriplePattern<SU, PR, VR>
where
    SU: SPQLVar + QueryFragment,
    PR: SPQLVar + QueryFragment,
    VR: SPQLVar + QueryFragment,
{
}

///
/// Traits to represent types used to
/// encode FILTER statements in Sparql
///
pub trait FilterStatementTrait {}
pub trait FilterConditionTrait {}
