use crate::graph_specifier::{GraphIdent, GraphSpecifier};
use crate::query_build::{QueryBuilder, QueryFragment};
use crate::update_types::{UpdateSelection, UpdateSelectionTrait, UpdateWhereClause};
use crate::where_clause::{WhereClauseTrait, WherePredicateSet};
use crate::prefix::{SPQLPrefixTrait, NullPrefixSet};

///
/// A marker trait for types which
/// represent an `InsertStatement`, in SPARQL
///
pub trait InsertStatementTrait {}

pub struct InsertStatement<PRE: SPQLPrefixTrait, G: GraphSpecifier, SEL: UpdateSelectionTrait, WHERE: WhereClauseTrait> {
    prefix: PRE,
    graph: G,
    selection: SEL,
    where_clause: WHERE,
}

impl<PRE, G, SEL, WHERE> InsertStatementTrait for InsertStatement<PRE, G, SEL, WHERE>
where
    PRE: SPQLPrefixTrait,
    G: GraphSpecifier,
    SEL: UpdateSelectionTrait,
    WHERE: WhereClauseTrait,
{
}

impl<PRE, G, SEL, WHERE> QueryFragment for InsertStatement<PRE, G, SEL, WHERE>
where
    PRE: SPQLPrefixTrait,
    G: QueryFragment + GraphSpecifier,
    SEL: QueryFragment + UpdateSelectionTrait,
    WHERE: QueryFragment + WhereClauseTrait,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("INSERT {\n");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.selection.generate_fragment(builder);
        builder.write_element("}}\n");
        self.where_clause.generate_fragment(builder);
    }
}

pub type Insert = InsertStatement<NullPrefixSet, GraphIdent, UpdateSelection, UpdateWhereClause>;

impl UpdateWhereClause {
    pub fn new(graph: impl std::string::ToString, predicates: WherePredicateSet) -> Self {
        Self {
            graph: GraphIdent::new(graph),
            predicates,
        }
    }
}

impl Insert {
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
mod insert_tests {

    use super::*;
    use crate::query_build::gen_fragment;
    use crate::triple_pattern::TriplePattern;
    use crate::where_clause::WherePredicateSet;

    #[test]
    fn gen_basic_insert() {
        let mut selection = UpdateSelection::new();
        selection.insert(Box::new(TriplePattern::new("foo", "bar", "baz")));

        let mut predicates = WherePredicateSet::new();
        predicates.insert_predicate(Box::new(TriplePattern::new("name", "foo", "bar")));
        let where_clause = UpdateWhereClause::new("bar", predicates);
        let insert = Insert::new("foo", selection, where_clause);

        let result = gen_fragment(insert);
        assert_eq!(
            result,
            "INSERT {\nGRAPH foo {\nfoo bar baz .\n}}\nWHERE {\nGRAPH bar {\nname foo bar .\n}}"
        );
    }
}
