use crate::graph_specifier::GraphSpecifier;
use crate::predicates::{Predicate, PredicateSet};
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
