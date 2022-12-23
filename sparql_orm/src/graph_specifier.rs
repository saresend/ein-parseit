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

///
/// A type to mark essentially that we want to leave the graph unspecified and instead use the
/// default graph
pub struct DefaultGraphSpecifier;

impl QueryFragment for DefaultGraphSpecifier {
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        // TODO: evaluate whether defaulting to using
        // a default graph called "default" makes sense as a behavior?
        builder.write_element("GRAPH default");
    }
}

use std::string::ToString;
impl GraphIdent {
    pub fn new(s: impl ToString) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod graph_spec_tests {
    use super::*;
    use crate::query_build::gen_fragment;
    #[test]
    fn test_graph_specifier() {
        let specifier = GraphIdent(String::from("test"));
        let result = gen_fragment(specifier);
        assert_eq!(result, "GRAPH test");
    }
}
