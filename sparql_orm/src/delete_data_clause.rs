//!
//! Delete Data Clause related implementation, which generates
//! Sparql `DELETE DATA` statements.
//!
//!


use crate::graph_specifier::{GraphIdent, GraphSpecifier};
use crate::query_build::{QueryBuilder, QueryFragment};
use crate::triple_pattern::{ConstTriple, SPQLConstTriple};

/// A marker trait for collections which
/// can be evaluate as part of a Delete Data statement
pub trait DeletableTripleSet {}

impl<CT, const N: usize> DeletableTripleSet for [CT; N] where CT: SPQLConstTriple {}

///
/// The main structure that holds
/// all of the elements to be deleted
///
pub struct DeleteDataClause<G: GraphSpecifier, CT: DeletableTripleSet> {
    graph: G,
    elems: CT,
}

impl<G, CT> QueryFragment for DeleteDataClause<G, CT>
where
    CT: QueryFragment + DeletableTripleSet,
    G: GraphSpecifier + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("DELETE DATA { ");
        self.graph.generate_fragment(builder);
        builder.write_element(" {\n");
        self.elems.generate_fragment(builder);
        builder.write_element("}}");
    }
}

pub trait SPQLMutableSet {
    type NewContainer;
    fn add_triple(self, trip: impl SPQLConstTriple) -> Self::NewContainer;
}

impl<CT, const N: usize> SPQLMutableSet for [CT; N]
where
    CT: SPQLConstTriple + Default,
    [CT; N + 1]: Sized
{
    type NewContainer = [CT; N + 1];
    fn add_triple(self, trip: impl SPQLConstTriple) -> Self::NewContainer where Self::NewContainer: Sized {
        todo!() 
    }
}

// Note: [T: N] already implements trait QueryFragment; no need to reimplement the trait here
pub type DeleteDataStatement<const N: usize> = DeleteDataClause<GraphIdent, [ConstTriple; N]>;

use std::string::ToString;

pub struct DeleteDataBuilder {}

#[doc(hidden)]
pub struct DeleteDataBuilderGraph<G: GraphSpecifier> {
    graph: G,
}

#[doc(hidden)]
pub struct DeleteDataBuilderComplete<G: GraphSpecifier, CT: DeletableTripleSet> {
    graph: G,
    elems: CT,
}

impl<G: GraphSpecifier> DeleteDataBuilderGraph<G> {
    pub fn triple<CT: DeletableTripleSet + Default + SPQLMutableSet>(
        self,
        triple: impl SPQLConstTriple,
    ) -> DeleteDataBuilderComplete<G, CT::NewContainer>
    where
        <CT as SPQLMutableSet>::NewContainer: DeletableTripleSet,
    {
        let mut elems = CT::default();
        let elems = elems.add_triple(triple);
        DeleteDataBuilderComplete {
            graph: self.graph,
            elems,
        }
    }
}

impl<G: GraphSpecifier, CT: DeletableTripleSet> DeleteDataBuilderComplete<G, CT> {
    /// Consumes the builder to create a DeleteDataClause  
    pub fn build(self) -> DeleteDataClause<G, CT> {
        DeleteDataClause {
            graph: self.graph,
            elems: self.elems,
        }
    }
}

impl DeleteDataBuilder {
    pub fn graph<G: GraphSpecifier>(graph: G) -> DeleteDataBuilderGraph<G> {
        DeleteDataBuilderGraph { graph }
    }
}

impl<const N: usize> DeleteDataStatement<N> {
    pub fn new(graph_name: impl ToString, elems: [ConstTriple; N]) -> Self {
        Self {
            graph: GraphIdent::new(graph_name),
            elems,
        }
    }

    /// Returns a builder for a delete data statement
    pub fn builder() -> DeleteDataBuilder {
        DeleteDataBuilder {}
    }
}
#[cfg(test)]
mod delete_data_tests {
    use super::*;
    use crate::query_build::gen_fragment;

    #[test]
    fn basic_delete_statement_test() {
        let del_statement = DeleteDataStatement::<3>::new(
            "test1",
            [
                ConstTriple::new("foo", "bar", "baz"),
                ConstTriple::new("bar", "baz", "foo"),
                ConstTriple::new("baz", "foo", "bar"),
            ],
        );
        let result = gen_fragment(del_statement);
        assert_eq!(
            result,
            "DELETE DATA { GRAPH test1 {
foo bar baz;
bar baz foo;
baz foo bar;
}}"
        );
    }
}
