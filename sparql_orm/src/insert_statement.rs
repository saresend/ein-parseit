use crate::triple_pattern::SPQLTriple;
use crate::where_clause::{WhereClauseTrait, WhereClause, WherePredicateSet};
use crate::graph_specifier::{GraphSpecifier, GraphIdent};
use crate::query_build::{QueryFragment, QueryBuilder};

///
/// A marker trait for types which 
/// represent an `InsertStatement`, in SPARQL
///
pub trait InsertStatementTrait {}

pub trait InsertSelectionTrait {}

pub struct InsertStatement<G: GraphSpecifier, SEL: InsertSelectionTrait, WHERE: WhereClauseTrait> 
{
    graph: G, 
    selection: SEL, 
    where_clause: WHERE,
}

impl<G, SEL, WHERE> InsertStatementTrait for InsertStatement<G, SEL, WHERE>
where G: GraphSpecifier, SEL: InsertSelectionTrait, WHERE: WhereClauseTrait
{
}

impl<G, SEL, WHERE> QueryFragment for InsertStatement<G, SEL, WHERE> where
G: QueryFragment + GraphSpecifier, 
SEL: QueryFragment + InsertSelectionTrait,
WHERE: QueryFragment + WhereClauseTrait {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("INSERT {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.selection.generate_fragment(builder);
        builder.write_element("}}\n");
        self.where_clause.generate_fragment(builder);
    }
}
 
pub struct InsertSelection {
    elems: Vec<Box<dyn SPQLTriple>>, 
}

impl InsertSelectionTrait for InsertSelection {}
impl QueryFragment for InsertSelection {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        for elem in &self.elems {
            elem.generate_fragment(builder); 
            builder.write_element(" .\n");
        }
    }
}

impl InsertSelection {
    pub fn new() -> Self {
        Self { elems: vec![] }
    }

    pub fn insert(&mut self, el: Box<dyn SPQLTriple>) {
        self.elems.push(el); 
    }

}

pub type Insert = InsertStatement<GraphIdent, InsertSelection, WhereClause<GraphIdent, WherePredicateSet>>;

pub type InsertWhereClause = WhereClause<GraphIdent, WherePredicateSet>;

impl InsertWhereClause {
    pub fn new(graph: impl std::string::ToString, predicates: WherePredicateSet) -> Self {
        Self { graph: GraphIdent::new(graph), predicates }
    }
}

impl Insert {
    pub fn new(graph: impl std::string::ToString, selection: InsertSelection, where_clause: InsertWhereClause) -> Self {
        Self { graph: GraphIdent::new(graph), selection, where_clause } 
    }
}

#[cfg(test)]
mod insert_tests {

    use super::*;
    use crate::triple_pattern::TriplePattern;
    use crate::where_clause::WherePredicateSet;
    use crate::query_build::gen_fragment;

    #[test]
    fn gen_basic_insert() {

        let mut selection = InsertSelection::new();
        selection.insert(Box::new(TriplePattern::new("foo", "bar", "baz")));

        let mut predicates = WherePredicateSet::new();
        predicates.insert_predicate(Box::new(TriplePattern::new("name", "foo", "bar")));
        let where_clause = InsertWhereClause::new("bar", predicates);
        let insert = Insert::new(
            "foo", 
            selection,
            where_clause,
        );

        let result = gen_fragment(insert);
        assert_eq!(result, "INSERT {\nGRAPH foo {\nfoo bar baz .\n}}\nWHERE {\nGRAPH bar {\nname foo bar .\n}}");

    }

}
