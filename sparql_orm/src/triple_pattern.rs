//!
//! Provides top-level traits and markers
//! for types implementing sparql triples
//!
//!
//! Ideal end state: have a type like
//! pub type Triple<Subject, Predicate, Object>, that we
//! can then build like Triple<Var<Binding>, Literal<LiteralBinding>, Var<Object>

use crate::sparql_var::SPQLVar;
use std::marker::PhantomData;

/// This is a marker trait to denote types that represent any valid
/// triple, or triple pattern
pub trait SPQLTriple {}

/// A marker for types that represent a triple with no variable bindings 
pub trait SPQLConstTriple {}

impl<T: SPQLConstTriple> SPQLTriple for T {}

struct TriplePattern<Subject: SPQLVar, Predicate: SPQLVar, Object: SPQLVar> {
    subject  : PhantomData<Subject>,
    predicate : PhantomData<Predicate>,
    object: PhantomData<Object>,
}

impl<SU, PR, OBJ> SPQLTriple for TriplePattern<SU, PR, OBJ>
where
    SU: SPQLVar,
    PR: SPQLVar,
    OBJ: SPQLVar,
{
}
