//!
//! Basic types to represent a single SPARQL Identifier
//! i.e. `name`, or `foaf:author`,
//!
use crate::query_build::{QueryFragment, SparqlQuery};

pub type Ident = String;

pub trait Identifier {
    fn gen_identifier(&self) -> Ident;
}

impl Identifier for Ident {
    fn gen_identifier(&self) -> Ident {
        self.clone()
    }
}
use crate::query_build::QueryBuilder;

impl<T> QueryFragment for T
where
    T: Identifier,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element(&self.gen_identifier());
    }
}
