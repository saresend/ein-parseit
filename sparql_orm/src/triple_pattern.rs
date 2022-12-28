//!
//! Provides top-level traits and markers
//! for types implementing sparql triples
//!
//!
//! Ideal end state: have a type like
//! pub type Triple<Subject, Predicate, Object>, that we
//! can then build like Triple<Var<Binding>, Literal<LiteralBinding>, Var<Object>

use crate::query_build::{QueryBuilder, QueryFragment};
use crate::sparql_var::{ConstVar, SPQLVar};

/// This is a marker trait to denote types that represent any valid
/// triple, or triple pattern
pub trait SPQLTriple: QueryFragment {}

/// A marker for types that represent a triple with no variable bindings
pub trait SPQLConstTriple {}

///
/// This implements 'ConstTriple' for all Triples that only contain
/// Constants / Literals in them
impl<SU, PR, OBJ> SPQLConstTriple for TriplePattern<SU, PR, OBJ>
where
    SU: ConstVar + SPQLVar,
    PR: ConstVar + SPQLVar,
    OBJ: ConstVar + SPQLVar,
{
}

pub struct TriplePattern<Subject: SPQLVar, Predicate: SPQLVar, Object: SPQLVar> {
    subject: Subject,
    predicate: Predicate,
    object: Object,
}

impl<SU, PR, OBJ> SPQLTriple for TriplePattern<SU, PR, OBJ>
where
    SU: SPQLVar + QueryFragment,
    PR: SPQLVar + QueryFragment,
    OBJ: SPQLVar + QueryFragment,
{
}

impl<SU, PR, OBJ> QueryFragment for TriplePattern<SU, PR, OBJ>
where
    SU: SPQLVar + QueryFragment,
    PR: SPQLVar + QueryFragment,
    OBJ: SPQLVar + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        self.subject.generate_fragment(builder);
        builder.write_element(" ");
        self.predicate.generate_fragment(builder);
        builder.write_element(" ");
        self.object.generate_fragment(builder);
    }
}

/*
 * Util Methods
 */

use crate::identifier::Ident;
use crate::sparql_var::Literal;
use std::string::ToString;

pub type ConstTriple = TriplePattern<Literal<Ident>, Literal<Ident>, Literal<Ident>>;

impl TriplePattern<Literal<Ident>, Literal<Ident>, Literal<Ident>> {
    pub fn new(sub: impl ToString, pred: impl ToString, obj: impl ToString) -> Self {
        Self {
            subject: Literal::<Ident>::new(sub),
            predicate: Literal::<Ident>::new(pred),
            object: Literal::<Ident>::new(obj),
        }
    }
}

#[cfg(test)]
mod triple_pattern_tests {
    use crate::identifier::Ident;
    use crate::sparql_var::{Literal, Variable};
    use crate::{
        query_build::gen_fragment,
        triple_pattern::{ConstTriple, SPQLConstTriple, TriplePattern},
    };
    #[test]
    fn test_literal_triple() {
        let triple = ConstTriple::new("foo", "bar", "baz");
        let result = gen_fragment(triple);
        assert_eq!(result, "foo bar baz");
    }

    #[test]
    fn test_var_triple() {
        let triple = TriplePattern {
            subject: Literal {
                v: Ident(String::from("foo")),
            },
            predicate: Variable {
                v: Ident(String::from("bar")),
            },
            object: Variable {
                v: Ident(String::from("baz")),
            },
        };

        let result = gen_fragment(triple);
        assert_eq!(result, "foo ?bar ?baz");
    }

    fn test_const_trip<T>()
    where
        T: SPQLConstTriple,
    {
    }

    #[test]
    fn assert_const_triple_impl() {
        test_const_trip::<TriplePattern<Literal<Ident>, Literal<Ident>, Literal<Ident>>>();
    }

    #[test]
    fn assert_type_alias_impl() {
        test_const_trip::<ConstTriple>();
    }
}
