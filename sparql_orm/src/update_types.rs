use crate::triple_pattern::SPQLTriple;
use crate::query_build::{QueryBuilder, QueryFragment}; 
use crate::graph_specifier::GraphIdent;
use crate::where_clause::{WhereClause, WherePredicateSet};

pub trait UpdateSelectionTrait {}

pub struct UpdateSelection {
    elems: Vec<Box<dyn SPQLTriple>>,
}

impl UpdateSelectionTrait for UpdateSelection {}

impl UpdateSelection {
    pub fn new() -> Self {
        Self { elems: vec![] }
    }

    pub fn insert(&mut self, el: Box<dyn SPQLTriple>) {
        self.elems.push(el);
    }
}


impl QueryFragment for UpdateSelection {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        for elem in &self.elems {
            elem.generate_fragment(builder);
            builder.write_element(" .\n");
        }
    }
}

pub type UpdateWhereClause = WhereClause<GraphIdent, WherePredicateSet>;
