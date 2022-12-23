//!
//! Basic types for handling graph specifiers in sparql
//!
use crate::query_build::QueryFragment;

#[derive(Clone, Debug)]
pub struct GraphIdent(String);

/// A trait that marks any type that can represent a graph specifier
pub trait GraphSpecifier {
    fn gen_specifier(&self) -> GraphIdent;
}

impl GraphSpecifier for GraphIdent {
    //TODO - Improve lifetime handling here 
    fn gen_specifier(&self) -> GraphIdent {
        self.clone()
    }
}

use crate::query_build::QueryBuilder;
impl QueryFragment for GraphIdent {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("GRAPH");
        builder.write_element(" ");
        builder.write_element(&self.gen_specifier().0);
    }    
}


#[cfg(test)]
mod graph_spec_tests {
    use crate::query_build::gen_fragment;
    use super::*;
    #[test]
    fn test_graph_specifier() {
        let specifier = GraphIdent(String::from("test"));
        let result = gen_fragment(specifier);
        assert_eq!(result, "GRAPH test");
    }

}
