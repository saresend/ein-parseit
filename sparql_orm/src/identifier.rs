//!
//! Basic types to represent a single SPARQL Identifier
//! i.e. `name`, or `foaf:author`,
//!
use crate::query_build::{QueryFragment, SparqlQuery};

#[derive(Clone)]
pub struct Ident(pub(crate) String);

pub trait Identifier {
    fn gen_identifier(&self) -> Ident;
}

impl Identifier for Ident {
    fn gen_identifier(&self) -> Ident {
        self.clone()
    }
}
use crate::query_build::QueryBuilder;

impl QueryFragment for Ident
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element(&self.gen_identifier().0);
    }
}

#[cfg(test)]
mod ident_tests {
    use crate::query_build::gen_fragment;
    use super::Ident; 
    #[test]
    fn test_ident_generation() {
        let ident: Ident = Ident(String::from("test"));
        let result = gen_fragment(ident);
        assert_eq!(result, "test");
    }
}
