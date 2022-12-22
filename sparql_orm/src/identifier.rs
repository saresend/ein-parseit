
//!
//! Basic types to represent a single SPARQL Identifier
//! i.e. `name`, or `foaf:author`,
//!

pub type Ident = String;

pub trait Identifier {
    fn gen_identifier(&self) -> Ident;
}

impl Identifier for Ident {
    fn gen_identifier(&self) -> Ident {
        self.clone()
    }
}

