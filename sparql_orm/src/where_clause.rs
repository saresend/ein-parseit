use crate::graph_specifier::GraphSpecifier;
use crate::predicates::PredicateSet;
use crate::query_build::{QueryBuilder, QueryFragment};
///
/// A trait to mark which types 
/// can be used to represent a where clause
pub trait WhereClauseTrait {}


pub struct WhereClause<G: GraphSpecifier, PRED: PredicateSet> {
    graph: G, 
    predicates: PRED
}


impl<G, PRED> QueryFragment for WhereClause<G, PRED> 
where G: GraphSpecifier + QueryFragment,
      PRED: PredicateSet + QueryFragment 
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("WHERE {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.predicates.generate_fragment(builder);
        builder.write_element("}}");
    }
}
