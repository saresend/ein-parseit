use crate::triple_pattern::SPQLTriple;
use crate::where_clause::WhereClauseTrait;
use crate::graph_specifier::GraphSpecifier;
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
