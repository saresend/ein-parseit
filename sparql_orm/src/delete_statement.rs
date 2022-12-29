use crate::where_clause::WhereClauseTrait;
use crate::query_build::{QueryBuilder, QueryFragment};
use crate::update_types::{UpdateSelectionTrait, UpdateSelection, UpdateWhereClause};
use crate::graph_specifier::{GraphSpecifier, GraphIdent};

///
/// A trait marker for types that represent a full `DELETE` statement
///
pub trait DeleteStatementTrait  {}

pub struct DeleteStatement<G: GraphSpecifier, SEL: UpdateSelectionTrait, WHERE: WhereClauseTrait> {
    graph: G, 
    selection: SEL, 
    where_clause: WHERE, 
}

impl<G, SEL, WHERE> QueryFragment for DeleteStatement<G, SEL, WHERE> 
where G: GraphSpecifier + QueryFragment, 
      SEL: UpdateSelectionTrait + QueryFragment, 
      WHERE: QueryFragment + WhereClauseTrait  {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("DELETE {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.selection.generate_fragment(builder);
        builder.write_element("}}\n");
        self.where_clause.generate_fragment(builder);
    }
}
pub type Delete = DeleteStatement<GraphIdent, UpdateSelection, UpdateWhereClause>;

impl Delete {
    pub fn new(
        graph: impl std::string::ToString,
        selection: UpdateSelection,
        where_clause: UpdateWhereClause,
    ) -> Self {
        Self {
            graph: GraphIdent::new(graph),
            selection,
            where_clause,
        }
    }
}

#[cfg(test)]
mod delete_tests {
    use super::*;
    use crate::query_build::gen_fragment;
    use crate::triple_pattern::TriplePattern;
    use crate::where_clause::WherePredicateSet;

    #[test]
    fn gen_basic_delete() {

        let mut selection = UpdateSelection::new();
        selection.insert(Box::new(TriplePattern::new("foo", "bar", "baz")));

        let mut predicates = WherePredicateSet::new();
        predicates.insert_predicate(Box::new(TriplePattern::new("name", "foo", "bar")));
        let where_clause = UpdateWhereClause::new("bar", predicates);
        let delete = Delete::new("foo", selection, where_clause); 
        let result = gen_fragment(delete);
        assert_eq!(
            result,
            "DELETE {\nGRAPH foo {\nfoo bar baz .\n}}\nWHERE {\nGRAPH bar {\nname foo bar .\n}}"
        );
    }

}
