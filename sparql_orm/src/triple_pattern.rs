//!
//! Provides top-level traits and markers
//! for types implementing sparql triples
//!
//!
//! Ideal end state: have a type like
//! pub type Triple<Subject, Predicate, Object>, that we
//! can then build like Triple<Var<Binding>, Literal<LiteralBinding>, Var<Object>

use crate::query_build::QueryFragment;
use crate::sparql_var::{ConstVar, SPQLVar};

/// This is a marker trait to denote types that represent any valid
/// triple, or triple pattern
pub trait SPQLTriple {}

/// A marker for types that represent a triple with no variable bindings
pub trait SPQLConstTriple {}


///
/// This implements 'ConstTriple' for all Triples that only contain
/// Constants / Literals in them
impl<SU, PR, OBJ> SPQLConstTriple for TriplePattern<SU, PR, OBJ> where SU: ConstVar + SPQLVar, PR: ConstVar + SPQLVar, OBJ: ConstVar + SPQLVar {}

struct TriplePattern<Subject: SPQLVar, Predicate: SPQLVar, Object: SPQLVar> {
    subject: Subject,
    predicate: Predicate,
    object: Object,
}

impl<SU, PR, OBJ> SPQLTriple for TriplePattern<SU, PR, OBJ>
where
    SU: SPQLVar,
    PR: SPQLVar,
    OBJ: SPQLVar,
{
}

use crate::query_build::QueryBuilder;

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

#[cfg(test)]
mod triple_pattern_tests {
    use crate::sparql_var::{Literal, Variable};
    use crate::{query_build::gen_fragment, triple_pattern::{SPQLConstTriple, TriplePattern}};
    use crate::identifier::Ident;
    #[test]
    fn test_literal_triple() {
        let triple = TriplePattern {
            subject: Literal {
                v: Ident(String::from("foo")),
            },
            predicate: Literal {
                v: Ident(String::from("bar")),
            },
            object: Literal {
                v: Ident(String::from("baz")),
            },
        };
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

    fn test_const_trip<T>() where T: SPQLConstTriple {}

    #[test]
    fn assert_const_triple_impl() {
        test_const_trip::<TriplePattern<Literal<Ident>, Literal<Ident>, Literal<Ident>>>(); 

    }
}
