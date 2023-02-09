use crate::graph_specifier::{GraphIdent, GraphSpecifier};
use crate::query_build::{QueryBuilder, QueryFragment};
use crate::update_types::{UpdateSelection, UpdateSelectionTrait, UpdateWhereClause};
use crate::where_clause::WhereClauseTrait;
use crate::prefix::{NullPrefixSet, SPQLPrefixTrait};

///
/// A trait marker for types that represent a full `DELETE` statement
///
pub trait DeleteStatementTrait {}

pub struct DeleteStatement<PRE: SPQLPrefixTrait, G: GraphSpecifier, SEL: UpdateSelectionTrait, WHERE: WhereClauseTrait> {
    prefix: PRE, 
    graph: G,
    selection: SEL,
    where_clause: WHERE,
}

impl<PRE, G, SEL, WHERE> QueryFragment for DeleteStatement<PRE, G, SEL, WHERE>
where
    PRE: SPQLPrefixTrait,
    G: GraphSpecifier + QueryFragment,
    SEL: UpdateSelectionTrait + QueryFragment,
    WHERE: QueryFragment + WhereClauseTrait,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("DELETE {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.selection.generate_fragment(builder);
        builder.write_element("}}\n");
        self.where_clause.generate_fragment(builder);
    }
}

pub type Delete = DeleteStatement<NullPrefixSet, GraphIdent, UpdateSelection, UpdateWhereClause>;

impl Delete {
    pub fn new(
        graph: impl std::string::ToString,
        selection: UpdateSelection,
        where_clause: UpdateWhereClause,
    ) -> Self {
        Self {
            prefix: NullPrefixSet {  },
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
