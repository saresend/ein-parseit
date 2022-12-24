//!
//! A module containing insert clause related functionality, traits
//! and types

use crate::graph_specifier::{GraphIdent, GraphSpecifier};
use crate::identifier::*;
use crate::query_build::{QueryBuilder, QueryFragment, SparqlQuery};
use crate::triple_pattern::SPQLConstTriple;

/// A marker trait for types that can be evaluated as part of
/// a InsertDataClause. Not that this explicitly will not
/// include variable binding triple patterns,
/// as those are not supported by INSERT DATA
///
pub trait InsertableDataTripleSet {}

impl<CT, const N: usize> InsertableDataTripleSet for [CT; N] where CT: SPQLConstTriple {}

impl<CT, const N: usize> QueryFragment for [CT; N]
where
    CT: SPQLConstTriple + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        for elem in self.iter() {
            elem.generate_fragment(builder);
            builder.write_element(";\n");
        }
    }
}

pub struct InsertDataClause<G: GraphSpecifier, SEL: InsertableDataTripleSet> {
    graph_spec: G,
    selector: SEL,
}

/// Implemented so that
/// type system knows the output of this
/// should be safe to invoke against a database
impl<G, SEL> SparqlQuery for InsertDataClause<G, SEL>
where
    G: GraphSpecifier + QueryFragment,
    SEL: InsertableDataTripleSet + QueryFragment,
{
}

impl<G, SEL> QueryFragment for InsertDataClause<G, SEL>
where
    G: GraphSpecifier + QueryFragment,
    SEL: InsertableDataTripleSet + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("INSERT DATA { ");
        self.graph_spec.generate_fragment(builder);
        builder.write_element(" {\n");
        self.selector.generate_fragment(builder);
        builder.write_element("}}");
    }
}

use crate::triple_pattern::ConstTriple;

pub type InsertDataStatement<const N: usize> = InsertDataClause<GraphIdent, [ConstTriple; N]>;

use std::string::ToString;

impl<const N: usize> InsertDataClause<GraphIdent, [ConstTriple; N]> {
    pub fn new(graph_name: impl ToString, elems: [ConstTriple; N]) -> Self {
        Self {
            graph_spec: GraphIdent::new(graph_name),
            selector: elems,
        }
    }
}

#[cfg(test)]
mod insert_data_clause_tests {
    use super::*;
    use crate::query_build::gen_fragment;

    #[test]
    fn test_basic_insert_data() {
        let v = InsertDataStatement::<3>::new(
            "test_graph",
            [
                ConstTriple::new("foo", "bar", "baz"),
                ConstTriple::new("toast", "topping", "butter"),
                ConstTriple::new("pancake", "topping", "syrup"),
            ],
        );

        let result = gen_fragment(v);

        assert_eq!(
            result,
            "INSERT DATA { GRAPH test_graph {
foo bar baz;
toast topping butter;
pancake topping syrup;
}}"
        );
    }
}
