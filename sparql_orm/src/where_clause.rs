use crate::graph_specifier::GraphSpecifier;
use crate::predicates::{Predicate, PredicateSet};
use crate::query_build::{QueryBuilder, QueryFragment};
///
/// A trait to mark which types
/// can be used to represent a where clause
pub trait WhereClauseTrait {}

pub struct WhereClause<G: GraphSpecifier, PRED: PredicateSet> {
    pub(crate) graph: G,
    pub(crate) predicates: PRED,
}

impl<G, PRED> WhereClauseTrait for WhereClause<G, PRED>
where
    G: GraphSpecifier,
    PRED: PredicateSet,
{
}

impl<G, PRED> QueryFragment for WhereClause<G, PRED>
where
    G: GraphSpecifier + QueryFragment,
    PRED: PredicateSet + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("WHERE {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.predicates.generate_fragment(builder);
        builder.write_element("}}");
    }
}

pub struct WherePredicateSet {
    elems: Vec<Box<dyn Predicate>>,
}

impl WherePredicateSet {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }

    pub fn insert_predicate(&mut self, el: Box<dyn Predicate>) {
        self.elems.push(el);
    }
}

impl PredicateSet for WherePredicateSet {}
impl QueryFragment for WherePredicateSet {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        for elem in &self.elems {
            elem.generate_fragment(builder);
            builder.write_element(" .\n");
        }
    }
}

#[cfg(test)]
mod where_clause_tests {
    use super::*;
    use crate::graph_specifier::GraphIdent;
    use crate::query_build::gen_fragment;
    use crate::triple_pattern::ConstTriple;
    #[test]
    fn test_basic_where_clause() {
        let mut where_clause = WherePredicateSet::new();
        let where_predicate = ConstTriple::new("foo", "bar", "baz");
        where_clause.insert_predicate(Box::new(where_predicate));
        let where_clause = WhereClause {
            graph: GraphIdent::new("test1"),
            predicates: where_clause,
        };
        let result = gen_fragment(where_clause);
        assert_eq!(result, "WHERE {\nGRAPH test1 {\nfoo bar baz .\n}}");
    }
}
