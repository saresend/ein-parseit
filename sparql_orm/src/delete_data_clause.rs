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
        builder.write_element("{ ");
        self.elems.generate_fragment(builder);
        builder.write_element("}}");
    }
}

type DeleteDataStatement<const N: usize> = DeleteDataClause<GraphIdent, [ConstTriple; N]>;
