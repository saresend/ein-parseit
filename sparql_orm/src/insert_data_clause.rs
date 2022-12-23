//!
//! A module containing insert clause related functionality, traits
//! and types

use std::marker::PhantomData;
use crate::identifier::*;
use crate::triple_pattern::SPQLConstTriple;

/// A marker trait for types that can be evaluated as part of 
/// a InsertDataClause. Not that this explicitly will not 
/// include variable binding triple patterns, 
/// as those are not supported by INSERT DATA 
///
pub trait InsertableDataTripleSet {}

/// A trait that marks any type that can represent a graph specifier
pub trait GraphSpecifier {}

pub struct EmptyTripleSet;

pub struct InsertTripleSet<CT: SPQLConstTriple, RST: InsertableDataTripleSet> {
    ct: PhantomData<CT>,
    rst: PhantomData<RST>,
}

impl InsertableDataTripleSet for EmptyTripleSet {}

pub struct InsertDataClause<G: GraphSpecifier, SEL: InsertableDataTripleSet> {
    a: PhantomData<G>,
    b: PhantomData<SEL>,
}
