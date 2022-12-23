//!
//! Basic types for handling graph specifiers in sparql
//!

pub type GraphIdent = String;

/// A trait that marks any type that can represent a graph specifier
pub trait GraphSpecifier {
    fn gen_specifier(&self) -> GraphIdent;
}
