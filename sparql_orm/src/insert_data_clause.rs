//!
//! A module containing insert clause related functionality, traits
//! and types

use crate::graph_specifier::GraphSpecifier;
use crate::identifier::*;
use crate::query_build::{QueryBuilder, QueryFragment, SparqlQuery};
use crate::triple_pattern::SPQLConstTriple;

/// A marker trait for types that can be evaluated as part of
/// a InsertDataClause. Not that this explicitly will not
/// include variable binding triple patterns,
/// as those are not supported by INSERT DATA
///
pub trait InsertableDataTripleSet {}
pub struct EmptyTripleSet;

pub struct InsertTripleSet<CT: SPQLConstTriple, RST: InsertableDataTripleSet> {
    ct: CT,
    rst: RST,
}

impl<CT, RST> InsertableDataTripleSet for InsertTripleSet<CT, RST> where
    CT: SPQLConstTriple,
    RST: InsertableDataTripleSet
{}

impl InsertableDataTripleSet for EmptyTripleSet {}

impl<CT, RST> QueryFragment for InsertTripleSet<CT, RST> where
CT: SPQLConstTriple + QueryFragment,
RST: InsertableDataTripleSet + QueryFragment,
{
    
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        self.ct.generate_fragment(builder);
        builder.write_element(";\n");
        self.rst.generate_fragment(builder);
    }
}

impl QueryFragment for EmptyTripleSet {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        // no - op
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

impl<G, SEL> QueryFragment for InsertDataClause<G, SEL> where
G: GraphSpecifier + QueryFragment,
SEL: InsertableDataTripleSet + QueryFragment {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("INSERT DATA {"); 
        self.graph_spec.generate_fragment(builder);
        builder.write_element("{");
        self.selector.generate_fragment(builder);
        builder.write_element("}}");
    }

}



#[cfg(test)]
mod insert_data_clause_tests {
    use super::*;
    use crate::query_build::gen_fragment;

    
    #[test]
    fn test_basic_insert_data() {
        
    }
}
